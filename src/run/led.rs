use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::DynPin;

use crate::{
	run::morse::code::{Code, Mark},
	BUFFER_LENGTH,
};

const TIME_UNIT: u32 = 200;

/// Blink led based on provided morse code
///
/// # Arguments
/// * `led` - The LED to blink
/// * `delay` - The system delay
/// * `codes` - The morse code to blink
pub fn blink_codes(led: &mut DynPin, delay: &mut Delay, codes: &[Code; BUFFER_LENGTH]) {
	for code in codes {
		match code {
			Code::Letter(code) => {
				for mark in code {
					match mark {
						Mark::Dot => {
							cycle_led(led, delay, TIME_UNIT);
						}
						Mark::Dash => {
							cycle_led(led, delay, TIME_UNIT * 3);
						}
						Mark::None => {}
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

/// Cycles an LED on and off for a specified duration
fn cycle_led(led: &mut DynPin, delay: &mut Delay, duration: u32) {
	led.set_high().unwrap();
	delay.delay_ms(duration);
	led.set_low().unwrap();
	delay.delay_ms(TIME_UNIT);
}
