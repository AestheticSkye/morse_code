#![no_std]
#![no_main]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod button;
mod initialization;
mod led;
mod morse;
mod pins;
mod serial;

use cortex_m::delay::Delay;
use embedded_hal::digital::v2::InputPin;
use rp2040_hal::usb::UsbBus;
use rp_pico::entry;
use usb_device::device::UsbDevice;
use usbd_serial::SerialPort;

use crate::{
	button::scan,
	initialization::{initialize_system, initialize_usb},
	led::blink_codes,
	morse::{codes_to_string, string_to_codes, to_marks},
	pins::PinSet,
	serial::read,
};

const BUFFER_LENGTH: usize = 64;

#[entry]
fn main() -> ! {
	let mut initialised = false;

	let (usb_bus, mut delay, timer, mut pin_set) = initialize_system();
	let (mut serial, mut usb_dev) = initialize_usb(&usb_bus);

	loop {
		// No clue why this has to be done, but serial wont work without it
		if !initialised && timer.get_counter().ticks() >= 2_000_000 {
			initialised = true;
			serial.write(b"Hello World!\r\n").unwrap();
		}

		usb_dev.poll(&mut [&mut serial]);

		if initialised {
			let mut current_on = 0;

			serial
				.write(b"Press button to select button mode.")
				.unwrap();

			new_line(&mut serial, &mut delay);

			serial.write(b"Hold button to select serial mode.").unwrap();

			new_line(&mut serial, &mut delay);

			loop {
				if pin_set.button.is_high().unwrap() {
					current_on += 1;
					if current_on == 300 {
						serial.write(b"Serial mode selected.\n\r").unwrap();
					}
				} else if current_on > 300 {
					run_serial(&mut pin_set, &mut delay, &mut serial, &mut usb_dev);
				} else if current_on > 0 {
					serial.write(b"Button mode selected.\n\r").unwrap();
					run_button(&mut pin_set, &mut delay, &mut serial);
				}
				delay.delay_ms(1);
			}
		}
	}
}

fn run_button(pin_set: &mut PinSet, delay: &mut Delay, serial: &mut SerialPort<UsbBus>) {
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

fn run_serial(
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

fn new_line(serial: &mut SerialPort<UsbBus>, delay: &mut Delay) {
	// Delays buy smallest time possible as without sometimes serial doesnt write properly
	delay.delay_ms(1);
	serial.write(&[b'\n', b'\r']).unwrap();
}
