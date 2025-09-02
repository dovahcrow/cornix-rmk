use defmt::unwrap;
use embassy_nrf::{
    Peri, PeripheralType,
    gpio::{Level, Output, OutputDrive, Pin},
    pwm::{
        Config, Instance, Prescaler, SequenceConfig, SequenceLoad, SequencePwm, SingleSequenceMode,
        SingleSequencer,
    },
};
use embassy_time::Duration;
use rmk::{
    channel::{CONTROLLER_CHANNEL, ControllerSub},
    controller::{Controller, PollingController},
    event::ControllerEvent,
};

const T1H: u16 = 0x0 | 13; // Duty = 13/20 ticks (0.8us/1.25us) for a 1
const T0H: u16 = 0x0 | 7; // Duty 7/20 ticks (0.4us/1.25us) for a 0
const RES: u16 = 0x0;

pub struct LedController<'d, T>
where
    T: PeripheralType + Instance,
{
    sub: ControllerSub,
    pwm: SequencePwm<'d, T>,
    seq_config: SequenceConfig,
    color_bit: usize,
    bit_value: u16,
    seq_words: [u16; 10],
}

impl<'d, T> LedController<'d, T>
where
    T: PeripheralType + Instance,
{
    pub fn new<D>(pwm: Peri<'d, T>, ch0: Peri<'d, D>, en: Peri<'d, impl Pin>) -> Self
    where
        D: Pin,
    {
        let mut config = Config::default();
        config.sequence_load = SequenceLoad::Common;
        config.prescaler = Prescaler::Div1;
        config.max_duty = 20; // 1.25us (1s / 16Mhz * 20)
        // let pwm = unwrap!(SequencePwm::new_1ch(pwm, ch0, config));
        let pwm = unwrap!(SequencePwm::new_1ch(pwm, ch0, config));

        Output::new(en, Level::Low, OutputDrive::Standard).set_low();

        let seq_words = [
            T0H, T0H, T0H, // G
            T0H, T0H, T0H, // R
            T1H, T1H, T1H, // B
            RES,
        ];
        let mut seq_config = SequenceConfig::default();
        seq_config.end_delay = 799; // 50us (20 ticks * 40) - 1 tick because we've already got one RES;

        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            pwm,
            seq_config,
            color_bit: 16,
            bit_value: T0H,
            seq_words,
        }
    }
}

impl<'a, T> Controller for LedController<'a, T>
where
    T: PeripheralType + Instance,
{
    type Event = ControllerEvent;

    async fn process_event(&mut self, event: Self::Event) {
        match event {
            ControllerEvent::Key(..) => {}
            _ => {}
        }
    }

    async fn next_message(&mut self) -> Self::Event {
        self.sub.next_message_pure().await
    }
}

impl<'a, T> PollingController for LedController<'a, T>
where
    T: PeripheralType + Instance,
{
    const INTERVAL: Duration = Duration::from_millis(100);

    async fn update(&mut self) {
        let sequences =
            SingleSequencer::new(&mut self.pwm, &self.seq_words, self.seq_config.clone());
        unwrap!(sequences.start(SingleSequenceMode::Infinite));

        drop(sequences);

        // self.seq_words[self.color_bit] = self.bit_value;
    }
}
