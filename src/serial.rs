use core::fmt::Write;
use heapless::String;
use rp2040_hal::usb::UsbBus;
use usb_device::device::UsbDevice;
use usbd_serial::SerialPort;

pub fn read(usb_dev: &mut UsbDevice<UsbBus>, serial: &mut SerialPort<UsbBus>) -> String<64> {
    let mut buffer_index = 0;
    let mut buffer = [0u8; 64];
    loop {
        if usb_dev.poll(&mut [serial]) {
            let mut current_buffer = [0u8; 64];
            if let Ok(count) = serial.read(&mut current_buffer) {
                let count = count.min(64 - buffer_index);

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
            return create_return_string("Buffer length reached.", buffer, serial);
        }
        if buffer.contains(&b'\n') || buffer.contains(&b'\r') {
            return create_return_string("Message submitted.", buffer, serial);
        }
    }

    fn remove_control_chars(buffer: &mut [u8; 64]) {
        for char in buffer.iter_mut() {
            if char == &b'\n' || char == &b'\r' {
                *char = 0
            }
        }
    }

    fn create_return_string(
        message: &str,
        mut buffer: [u8; 64],
        serial: &mut SerialPort<UsbBus>,
    ) -> String<64> {
        let mut string = String::<64>::new();

        remove_control_chars(&mut buffer);

        for byte in buffer {
            if byte != 0 {
                string.push(byte as char).unwrap();
            }
        }

        let mut formatted_message = String::<128>::new();

        write!(
            &mut formatted_message,
            "\n{}\nNow encoding '{}' to morse.\n",
            message, string
        )
        .unwrap();

        // Adds space to end of string for blinking
        for (index, byte) in buffer.iter().enumerate() {
            if let Some(next_byte) = buffer.get(index + 1) {
                if *next_byte == 0 && *byte != b' ' {
                    string.push(' ').unwrap();
                    break;
                }
            }
        }

        serial.write(formatted_message.as_bytes()).unwrap();

        string
    }
}
