use crate::morse::code::Mark::{Dash, Dot};
use crate::morse::code::{Code, Mark};
use crate::pins::PinSet;
use crate::BUFFER_LENGTH;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use heapless::Vec;
use rp2040_hal::usb::UsbBus;
use usbd_serial::SerialPort;

const LONG_PRESS_LENGTH: u32 = 500;
const PASSAGE_END_LENGTH: u32 = 1500;

const LETTER_TIME_LENGTH: u32 = 1000;
const WORD_TIME_LENGTH: u32 = 2000;

/// Scans the button for input and returns a morse code buffer
///
/// # Arguments
/// * `pin_set` - The pins to use for input and output
/// * `delay` - The system delay
/// * `serial` - The serial port
///
/// # Returns
/// * `[Code; BUFFER_LENGTH]` - The morse code buffer
///
pub fn scan(
	pin_set: &mut PinSet,
	delay: &mut Delay,
	serial: &mut SerialPort<UsbBus>,
) -> [Code; BUFFER_LENGTH] {
	let mut codes: Vec<Code, BUFFER_LENGTH> = Vec::new();
	let mut current_code: Vec<Mark, 5> = Vec::new();
	let mut current_mark: Mark = Mark::None;

	let mut button_on_time: u32 = 0;
	let mut button_off_time: u32 = 0;

	let mut passage_ended = false;

	loop {
		if pin_set.button.is_high().unwrap() {
			button_on_event(
				&mut button_on_time,
				&mut button_off_time,
				&mut current_mark,
				&mut passage_ended,
				pin_set,
			);
		} else {
			if passage_ended {
				if !current_code.is_empty() {
					handle_letter(&mut codes, &mut current_code, pin_set);
				}
				break;
			}

			if button_on_time > 0 {
				handle_mark(&mut current_code, &mut current_mark, &mut codes, serial);
			}

			if button_off_time == LETTER_TIME_LENGTH {
				handle_letter(&mut codes, &mut current_code, pin_set);
			}

			if button_off_time == WORD_TIME_LENGTH {
				handle_word(pin_set, serial, &mut codes);
			}

			pin_set.short_press_led.set_low().unwrap();
			pin_set.long_press_led.set_low().unwrap();

			button_off_time += 100;
			button_on_time = 0;
		}

		delay.delay_ms(100);
	}

	pin_set.leds_off();

	finalise_codes(codes)
}

/// Handles button being pressed
fn button_on_event(
	button_on_time: &mut u32,
	button_off_time: &mut u32,
	current_mark: &mut Mark,
	passage_ended: &mut bool,
	pin_set: &mut PinSet,
) {
	pin_set.short_press_led.set_high().unwrap();
	*current_mark = Dot;
	if *button_on_time > LONG_PRESS_LENGTH {
		pin_set.long_press_led.set_high().unwrap();
		*current_mark = Dash;
	}
	if *button_on_time > PASSAGE_END_LENGTH {
		pin_set.passage_end_led.set_high().unwrap();

		pin_set.short_press_led.set_low().unwrap();
		pin_set.long_press_led.set_low().unwrap();

		*passage_ended = true;
	}

	pin_set.word_led.set_low().unwrap();
	pin_set.letter_led.set_low().unwrap();

	*button_on_time += 100;
	*button_off_time = 0;
}

/// Handles button release event for adding mark to current letter
fn handle_mark(
	current_code: &mut Vec<Mark, 5>,
	current_mark: &mut Mark,
	codes: &mut Vec<Code, BUFFER_LENGTH>,
	serial: &mut SerialPort<UsbBus>,
) {
	if current_code.is_full() {
		codes.push(Code::Error).unwrap();
		*current_code = Vec::new();
	} else {
		match *current_mark {
			Dot => {
				serial.write(b".").unwrap();
			}
			Dash => {
				serial.write(b"-").unwrap();
			}
			Mark::None => {}
		}
		current_code.push(*current_mark).unwrap();
	}
}

/// Handles button release event for finishing letter
fn handle_letter(
	codes: &mut Vec<Code, BUFFER_LENGTH>,
	current_code: &mut Vec<Mark, 5>,
	pin_set: &mut PinSet,
) {
	pin_set.letter_led.set_high().unwrap();

	// Fills rest of vec to be able to convert to array
	while !current_code.is_full() {
		current_code.push(Mark::None).unwrap();
	}

	codes
		.push(Code::Some(current_code.clone().into_array().unwrap()))
		.unwrap();
	*current_code = Vec::new();
}

/// Handles button release event for finishing word
fn handle_word(
	pin_set: &mut PinSet,
	serial: &mut SerialPort<UsbBus>,
	codes: &mut Vec<Code, BUFFER_LENGTH>,
) {
	pin_set.word_led.set_high().unwrap();
	serial.write(b" ").unwrap();
	codes.push(Code::Space).unwrap();
}

/// Formats finished code set properly
fn finalise_codes(mut codes: Vec<Code, BUFFER_LENGTH>) -> [Code; BUFFER_LENGTH] {
	// Adds space to end of code for blinking
	if let Some(last) = codes.last() {
		match last {
			Code::Space => {}
			_ => codes.push(Code::Space).unwrap(),
		}
	}

	// Fills rest of vec to be able to convert to array
	while !codes.is_full() {
		codes.push(Code::None).unwrap();
	}

	codes.into_array().unwrap()
}
