use crate::morse::code::Code;
use crate::BUFFER_LENGTH;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::DynPin;

const TIME_UNIT: u32 = 200;

pub fn blink_codes(led: &mut DynPin, delay: &mut Delay, codes: &[Code; BUFFER_LENGTH]) {
    for code in codes {
        match code {
            Code::Some(code) => {
                for code_number in code {
                    if *code_number == 1 {
                        // Standard says 1 unit for `dot`
                        cycle_led(led, delay, TIME_UNIT);
                    } else if *code_number == 2 {
                        // Standard says 3 units for `dash`
                        cycle_led(led, delay, TIME_UNIT * 3);
                    }
                }
                // Standard says 3 units for inter-element, a one unit delay already done when mark was deactivated.
                delay.delay_ms(TIME_UNIT * 2);
            }
            Code::Space => {
                // Standard says 7 units for inter-letter, same reason as above
                delay.delay_ms(TIME_UNIT);
            }
            _ => {}
        }
    }
}

fn cycle_led(led: &mut DynPin, delay: &mut Delay, duration: u32) {
    led.set_high().unwrap();
    delay.delay_ms(duration);
    led.set_low().unwrap();
    delay.delay_ms(TIME_UNIT);
}
