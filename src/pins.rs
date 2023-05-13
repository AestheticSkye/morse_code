use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::DynPin;

pub struct PinSet {
    pub internal_led: DynPin,
    pub letter_led: DynPin,
    pub word_led: DynPin,
    pub short_press_led: DynPin,
    pub long_press_led: DynPin,
    pub passage_end_led: DynPin,
    pub button: DynPin,
}

impl PinSet {
    pub const fn new(
        internal_led: DynPin,
        letter_led: DynPin,
        word_led: DynPin,
        short_press_led: DynPin,
        long_press_led: DynPin,
        passage_end_led: DynPin,
        button: DynPin,
    ) -> Self {
        Self {
            internal_led,
            letter_led,
            word_led,
            short_press_led,
            long_press_led,
            passage_end_led,
            button,
        }
    }

    pub fn leds_off(&mut self) {
        self.passage_end_led.set_low().unwrap();
        self.word_led.set_low().unwrap();
        self.letter_led.set_low().unwrap();
        self.short_press_led.set_low().unwrap();
        self.long_press_led.set_low().unwrap();
    }
}
