mod button;
mod led;
mod morse;
pub mod pins;
mod serial;

use cortex_m::delay::Delay;
use embedded_hal::digital::v2::InputPin;
use rp2040_hal::usb::UsbBus;
use usb_device::device::UsbDevice;
use usbd_serial::SerialPort;

use crate::{
	modes::{
		button::scan,
		led::blink_codes,
		morse::{codes_to_string, string_to_codes, to_marks},
		pins::PinSet,
		serial::read,
	},
	new_line,
};

pub fn run_button(pin_set: &mut PinSet, delay: &mut Delay, serial: &mut SerialPort<UsbBus>) {
	serial
		.write(b"Please press the button to start your message\r\n")
		.unwrap();

	while pin_set.button.is_low().unwrap() {}

	let codes = scan(pin_set, delay, serial);

	new_line(serial, delay);

	serial.write(codes_to_string(&codes).as_bytes()).unwrap();

	new_line(serial, delay);

	loop {
		blink_codes(&mut pin_set.internal_led, delay, &codes);
	}
}

pub fn run_serial(
	pin_set: &mut PinSet,
	delay: &mut Delay,
	serial: &mut SerialPort<UsbBus>,
	usb_dev: &mut UsbDevice<UsbBus>,
) {
	serial
		.write(b"Please enter the text you wish to encode into morse.\r\n")
		.unwrap();

	let converted_text = read(usb_dev, serial);

	let codes = string_to_codes(&converted_text);

	serial.write(to_marks(&codes).as_bytes()).unwrap();

	new_line(serial, delay);

	loop {
		blink_codes(&mut pin_set.internal_led, delay, &codes);
	}
}
