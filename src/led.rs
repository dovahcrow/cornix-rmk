use defmt::unwrap;
use embassy_nrf::{
    Peri, PeripheralType,
    gpio::{AnyPin, Level, Output, OutputDrive, Pin},
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

pub const T1H: u16 = 13; // Duty = 13/20 ticks (0.8us/1.25us) for a 1
pub const T0H: u16 = 7; // Duty 7/20 ticks (0.4us/1.25us) for a 0

pub struct LedController<'s, 'd, T>
where
    T: PeripheralType + Instance,
{
    sub: ControllerSub,
    sequencer: SingleSequencer<'s, 'd, T>,
}

impl<'s, 'd, T> LedController<'s, 'd, T>
where
    'd: 's,
    T: PeripheralType + Instance,
{
    pub fn new(pwm: &'s mut SequencePwm<'d, T>, seq_words: &'s [u16]) -> Self {
        let mut seq_config = SequenceConfig::default();
        seq_config.end_delay = 800; // 50us (20 ticks * 40) - 1 tick because we've already got one RES;

        let sequences = SingleSequencer::new(pwm, seq_words, seq_config);
        unwrap!(sequences.start(SingleSequenceMode::Infinite));

        Self {
            sub: unwrap!(CONTROLLER_CHANNEL.subscriber()),
            sequencer: sequences,
        }
    }
}

impl<'s, 'd, T> Controller for LedController<'s, 'd, T>
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

impl<'s, 'd, T> PollingController for LedController<'s, 'd, T>
where
    T: PeripheralType + Instance,
{
    const INTERVAL: Duration = Duration::from_millis(100);

    async fn update(&mut self) {
        // let sequences =
        //     SingleSequencer::new(&mut self.pwm, &self.seq_words, self.seq_config.clone());
        // unwrap!(sequences.start(SingleSequenceMode::Infinite));

        // drop(sequences);

        // self.seq_words[self.color_bit] = self.bit_value;
    }
}
