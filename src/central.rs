#![no_std]
#![no_main]

mod vial;
#[macro_use]
mod macros;
mod constants;
mod keymap;
mod led;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Input, Output, Pull};
use embassy_nrf::interrupt::{self, InterruptExt};
use embassy_nrf::mode::Async;
use embassy_nrf::peripherals::{RNG, SAADC, USBD};
use embassy_nrf::saadc::{self, AnyInput, Input as _, Saadc};
use embassy_nrf::usb::Driver;
use embassy_nrf::usb::vbus_detect::HardwareVbusDetect;
use embassy_nrf::{Peri, bind_interrupts, rng, usb};
use embassy_time::Duration;
use nrf_mpsl::Flash;
use nrf_sdc::mpsl::MultiprotocolServiceLayer;
use nrf_sdc::{self as sdc, mpsl};
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use rmk::ble::trouble::build_ble_stack;
use rmk::channel::EVENT_CHANNEL;
use rmk::config::{BehaviorConfig, BleBatteryConfig, RmkConfig, StorageConfig, TapHoldConfig};
use rmk::controller::PollingController;
use rmk::debounce::default_debouncer::DefaultDebouncer;
use rmk::futures::future::{join3, join4};
use rmk::input_device::Runnable;
use rmk::input_device::adc::{AnalogEventType, NrfAdc};
use rmk::input_device::battery::BatteryProcessor;
use rmk::input_device::rotary_encoder::RotaryEncoder;
use rmk::keyboard::Keyboard;
use rmk::morse::MorseMode;
use rmk::split::ble::central::read_peripheral_addresses;
use rmk::split::central::{CentralMatrix, run_peripheral_manager};
use rmk::{
    HostResources, initialize_encoder_keymap_and_storage, run_devices, run_processor_chain, run_rmk,
};
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

use crate::constants::{
    INPUT_PIN_NUM, KEYBOARD_USB_CONFIG, L2CAP_MTU, L2CAP_RXQ, L2CAP_TXQ, OUTPUT_PIN_NUM,
};
use crate::keymap::{COL, NUM_ENCODER, NUM_LAYER, ROW};
use crate::led::LedController;
use crate::vial::VIAL_CONFIG;

// embassy_nrf::interrupt_mod!(PWM0);

bind_interrupts!(struct Irqs {
    USBD => usb::InterruptHandler<USBD>;
    SAADC => saadc::InterruptHandler;
    RNG => rng::InterruptHandler<RNG>;
    EGU0_SWI0 => nrf_sdc::mpsl::LowPrioInterruptHandler;
    CLOCK_POWER => nrf_sdc::mpsl::ClockInterruptHandler, usb::vbus_detect::InterruptHandler;
    RADIO => nrf_sdc::mpsl::HighPrioInterruptHandler;
    TIMER0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
    RTC0 => nrf_sdc::mpsl::HighPrioInterruptHandler;
});

#[embassy_executor::task]
async fn mpsl_task(mpsl: &'static MultiprotocolServiceLayer<'static>) -> ! {
    mpsl.run().await
}

fn build_sdc<'d, const N: usize>(
    p: nrf_sdc::Peripherals<'d>,
    rng: &'d mut rng::Rng<RNG, Async>,
    mpsl: &'d MultiprotocolServiceLayer,
    mem: &'d mut sdc::Mem<N>,
) -> Result<nrf_sdc::SoftdeviceController<'d>, nrf_sdc::Error> {
    sdc::Builder::new()?
        .support_scan()?
        .support_central()?
        .support_adv()?
        .support_peripheral()?
        .support_dle_peripheral()?
        .support_dle_central()?
        .support_phy_update_central()?
        .support_phy_update_peripheral()?
        .support_le_2m_phy()?
        .central_count(1)?
        .peripheral_count(1)?
        .buffer_cfg(L2CAP_MTU as u16, L2CAP_MTU as u16, L2CAP_TXQ, L2CAP_RXQ)?
        .build(p, rng, mpsl, mem)
}

/// Initializes the SAADC peripheral in single-ended mode on the given pin.
fn init_adc(adc_pin: AnyInput, adc: Peri<'static, SAADC>) -> Saadc<'static, 1> {
    // Then we initialize the ADC. We are only using one channel in this example.
    let config = saadc::Config::default();
    let channel_cfg = saadc::ChannelConfig::single_ended(adc_pin.degrade_saadc());
    interrupt::SAADC.set_priority(interrupt::Priority::P3);
    let saadc = saadc::Saadc::new(adc, Irqs, config, [channel_cfg]);
    saadc
}

fn ble_addr() -> [u8; 6] {
    let ficr = embassy_nrf::pac::FICR;
    let high = u64::from(ficr.deviceid(1).read());
    let addr = high << 32 | u64::from(ficr.deviceid(0).read());
    let addr = addr | 0x0000_c000_0000_0000;
    unwrap!(addr.to_le_bytes()[..6].try_into())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello RMK BLE!");
    // Initialize the peripherals and nrf-sdc controller
    let mut nrf_config = embassy_nrf::config::Config::default();
    nrf_config.dcdc.reg0_voltage = Some(embassy_nrf::config::Reg0Voltage::_3V3);
    nrf_config.dcdc.reg0 = true;
    nrf_config.dcdc.reg1 = true;
    let p = embassy_nrf::init(nrf_config);
    let mpsl_p =
        mpsl::Peripherals::new(p.RTC0, p.TIMER0, p.TEMP, p.PPI_CH19, p.PPI_CH30, p.PPI_CH31);
    let lfclk_cfg = mpsl::raw::mpsl_clock_lfclk_cfg_t {
        source: mpsl::raw::MPSL_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_CTIV as u8,
        rc_temp_ctiv: mpsl::raw::MPSL_RECOMMENDED_RC_TEMP_CTIV as u8,
        accuracy_ppm: mpsl::raw::MPSL_DEFAULT_CLOCK_ACCURACY_PPM as u16,
        skip_wait_lfclk_started: mpsl::raw::MPSL_DEFAULT_SKIP_WAIT_LFCLK_STARTED != 0,
    };

    static MPSL: StaticCell<MultiprotocolServiceLayer> = StaticCell::new();
    static SESSION_MEM: StaticCell<mpsl::SessionMem<1>> = StaticCell::new();

    let mpsl = MPSL.init(unwrap!(mpsl::MultiprotocolServiceLayer::with_timeslots(
        mpsl_p,
        Irqs,
        lfclk_cfg,
        SESSION_MEM.init(mpsl::SessionMem::new())
    )));
    spawner.must_spawn(mpsl_task(&*mpsl));
    let sdc_p = sdc::Peripherals::new(
        p.PPI_CH17, p.PPI_CH18, p.PPI_CH20, p.PPI_CH21, p.PPI_CH22, p.PPI_CH23, p.PPI_CH24,
        p.PPI_CH25, p.PPI_CH26, p.PPI_CH27, p.PPI_CH28, p.PPI_CH29,
    );
    let mut rng = rng::Rng::new(p.RNG, Irqs);
    let mut rng_gen = ChaCha12Rng::from_rng(&mut rng).unwrap();
    let mut sdc_mem = sdc::Mem::<8192>::new();
    let sdc = unwrap!(build_sdc(sdc_p, &mut rng, mpsl, &mut sdc_mem));
    let mut host_resources = HostResources::new();
    let stack = build_ble_stack(sdc, ble_addr(), &mut rng_gen, &mut host_resources).await;

    // Initialize usb driver
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));

    // Initialize flash
    let flash = Flash::take(mpsl, p.NVMC);

    // Initialize IO Pins
    let (input_pins, output_pins) = config_matrix_pins_nrf!(peripherals: p, input: [P0_30, P0_31, P0_29, P0_02], output: [P0_28, P0_03, P1_10, P1_11, P1_13, P0_09, P0_10]);

    // Initialize the ADC.
    // We are only using one channel for detecting battery level
    let saadc = init_adc(p.P0_05.degrade_saadc() /* another name: AI3*/, p.SAADC);
    // Wait for ADC calibration.
    saadc.calibrate().await;

    // Keyboard config
    let storage_config = StorageConfig {
        start_addr: 0xA0000,
        num_sectors: 32,
        clear_storage: false,
        ..Default::default()
    };
    let rmk_config = RmkConfig {
        usb_config: KEYBOARD_USB_CONFIG,
        vial_config: VIAL_CONFIG,
        ble_battery_config: BleBatteryConfig::new(
            Some(Input::new(p.P1_09, Pull::Up)),
            false,
            None,
            false,
        ),
        storage_config,
    };

    // Initialze keyboard stuffs
    // Initialize the storage and keymap
    let mut behavior_config = BehaviorConfig {
        keyboard_macros: keymap::get_macros(),
        combo: keymap::get_combos(),
        morse: keymap::get_morses(),
        tap_hold: TapHoldConfig {
            enable_hrm: true,
            prior_idle_time: Duration::from_millis(30u64),
            timeout: Duration::from_millis(200u64),
            mode: MorseMode::PermissiveHold,
            unilateral_tap: true,
        },
        ..Default::default()
    };
    let mut keymap = keymap::get_keymap();
    let mut encoder_map = keymap::get_encoder_map();
    let (keymap, mut storage) = initialize_encoder_keymap_and_storage(
        &mut keymap,
        &mut encoder_map,
        flash,
        &storage_config,
        &mut behavior_config,
    )
    .await;

    let pin_a = Input::new(p.P1_06, embassy_nrf::gpio::Pull::None);
    let pin_b = Input::new(p.P1_04, embassy_nrf::gpio::Pull::None);
    let mut encoder = RotaryEncoder::with_resolution(pin_a, pin_b, 4, true, 0);

    // Initialize the matrix and keyboard
    let debouncer = DefaultDebouncer::<{ INPUT_PIN_NUM }, { OUTPUT_PIN_NUM }>::new();
    let mut matrix = CentralMatrix::<_, _, _, 0, 0, { INPUT_PIN_NUM }, { OUTPUT_PIN_NUM }>::new(
        input_pins,
        output_pins,
        debouncer,
    );
    // let mut matrix = TestMatrix::<ROW, COL>::new();
    let mut keyboard = Keyboard::new(&keymap);

    // Read peripheral address from storage
    let peripheral_addrs =
        read_peripheral_addresses::<1, _, { ROW }, { COL }, { NUM_LAYER }, { NUM_ENCODER }>(
            &mut storage,
        )
        .await;

    // Initialize the encoder processor
    let mut adc_device = NrfAdc::new(
        saadc,
        [AnalogEventType::Battery],
        embassy_time::Duration::from_secs(12),
        None,
    );
    let mut batt_proc = BatteryProcessor::new(2000, 2806, &keymap);

    let mut led_controller = LedController::new(p.PWM0, p.P0_24, p.P0_13);
    // Initialize the controllers
    // Start
    join4(
        run_devices! (
            (matrix, encoder, adc_device) => EVENT_CHANNEL,
        ),
        run_processor_chain! {
            EVENT_CHANNEL => [batt_proc],
        },
        keyboard.run(),
        join3(
            run_peripheral_manager::<INPUT_PIN_NUM, OUTPUT_PIN_NUM, 0, 7, _>(
                0,
                peripheral_addrs[0],
                &stack,
            ),
            run_rmk(&keymap, driver, &stack, &mut storage, rmk_config),
            led_controller.polling_loop(),
        ),
    )
    .await;
}
