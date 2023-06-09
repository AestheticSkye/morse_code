use core::fmt::Write;
use heapless::String;
use rp2040_hal::usb::UsbBus;
use usb_device::device::UsbDevice;
use usbd_serial::SerialPort;

use crate::BUFFER_LENGTH;

/// Reads from the serial port and returns the string
///
/// # Arguments
/// * `usb_dev` - The USB device
/// * `serial` - The serial port
///
/// # Returns
/// * `String<BUFFER_LENGTH>` - The string read from the serial port
pub fn read(
	usb_dev: &mut UsbDevice<UsbBus>,
	serial: &mut SerialPort<UsbBus>,
) -> String<BUFFER_LENGTH> {
	let mut buffer_index = 0;
	let mut buffer = [0u8; BUFFER_LENGTH];
	loop {
		if usb_dev.poll(&mut [serial]) {
			let mut current_buffer = [0u8; BUFFER_LENGTH];
			if let Ok(count) = serial.read(&mut current_buffer) {
				let count = count.min(BUFFER_LENGTH - buffer_index);

				buffer[buffer_index..(count + buffer_index)]
					.copy_from_slice(&current_buffer[..count]);

				buffer_index += count;

				// Send back to the host
				let mut wr_ptr = &current_buffer[..count];
				while !wr_ptr.is_empty() {
					match serial.write(wr_ptr) {
						Ok(len) => wr_ptr = &wr_ptr[len..],
						// On error, just drop unwritten data.
						// One possible error is Err(WouldBlock), meaning the USB
						// write buffer is full.
						Err(_) => break,
					};
				}
			}
		}
		if buffer.len() == buffer_index {
			return create_return_string("Buffer length reached.", buffer, buffer_index, serial);
		}
		if buffer.contains(&b'\n') || buffer.contains(&b'\r') {
			return create_return_string("Message submitted.", buffer, buffer_index, serial);
		}
	}
}

/// Creates the return string for the serial port & writes a message to the serial port
///
/// # Arguments
/// * `message` - The message to write
/// * `buffer` - The buffer to convert to a string
/// * `serial` - The serial port
///
/// # Returns
/// * `String<BUFFER_LENGTH>` - The string to return
fn create_return_string(
	message: &str,
	mut buffer: [u8; BUFFER_LENGTH],
	buffer_index: usize,
	serial: &mut SerialPort<UsbBus>,
) -> String<BUFFER_LENGTH> {
	let mut string = String::<BUFFER_LENGTH>::new();

	for byte in buffer.iter_mut() {
		// Remove linefeed and carriage return
		if byte == &b'\n' || byte == &b'\r' {
			*byte = 0;
		}
		// Add to string
		if *byte != 0 {
			string.push(*byte as char).unwrap();
		}
	}

	let mut formatted_message = String::<{ BUFFER_LENGTH * 2 }>::new();

	write!(
		&mut formatted_message,
		"\r\n{message}\r\nNow encoding '{string}' to morse.\r\n",
	)
	.unwrap();

	serial.flush().unwrap();

	serial.write(formatted_message.as_bytes()).unwrap();

	// Adds space to end of string for blinking
	if buffer.last().unwrap_or(&0) == &0 && buffer_index != BUFFER_LENGTH {
		buffer[buffer_index + 1] = b' ';
	}

	string
}
