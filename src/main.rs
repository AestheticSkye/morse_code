#![no_std]
#![no_main]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod initialization;
mod modes;

use cortex_m::delay::Delay;
use embedded_hal::digital::v2::InputPin;
use rp2040_hal::usb::UsbBus;
use rp_pico::entry;
use usbd_serial::SerialPort;

use crate::{
	initialization::{initialize_system, initialize_usb},
	modes::{run_button, run_serial},
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

fn new_line(serial: &mut SerialPort<UsbBus>, delay: &mut Delay) {
	// Delays buy smallest time possible as without sometimes serial doesnt write properly
	delay.delay_ms(1);
	serial.write(&[b'\n', b'\r']).unwrap();
}
