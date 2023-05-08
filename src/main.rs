#![no_std]
#![no_main]

mod led_manager;
mod morse;
mod serial;

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

use crate::led_manager::blink_codes;
use crate::morse::code::Code;
use crate::morse::string_to_codes;
use crate::serial::read;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::{clocks::Clock, gpio::DynPin, usb::UsbBus, Timer};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    #[cfg(feature = "rp2040-e5")]
    {
        let sio = hal::Sio::new(pac.SIO);
        let _pins = rp_pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );
    }

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut initialised = false;

    let mut internal_led: DynPin = pins.gpio25.into_push_pull_output().into();
    let mut letter_led: DynPin = pins.gpio15.into_push_pull_output().into();
    let mut word_led: DynPin = pins.gpio14.into_push_pull_output().into();
    let mut short_press_led: DynPin = pins.gpio16.into_push_pull_output().into();
    let mut long_press_led: DynPin = pins.gpio17.into_push_pull_output().into();
    let mut phrase_end_led: DynPin = pins.gpio18.into_push_pull_output().into();

    let button: DynPin = pins.gpio13.into_pull_down_input().into();

    loop {
        // No clue why this has to be done, but serial wont work without it
        if !initialised && timer.get_counter().ticks() >= 2_000_000 {
            initialised = true;
            serial.write("Hello World!\n".as_bytes()).unwrap();
        }

        usb_dev.poll(&mut [&mut serial]);

        if initialised {
            // delay.delay_ms(2000);
            // serial.write("test".as_bytes()).unwrap();
            // test_lights(
            //     &mut [
            //         &mut letter_led,
            //         &mut word_led,
            //         &mut short_press_led,
            //         &mut long_press_led,
            //         &mut phrase_end_led,
            //         &mut internal_led,
            //     ],
            //     &mut delay,
            // );

            serial
                .write("Please enter the text you wish to encode into morse.\n".as_bytes())
                .unwrap();

            let converted_text = read(&mut usb_dev, &mut serial);

            let codes = string_to_codes(&converted_text);

            for code in codes {
                match code {
                    Code::Some(code) => {
                        for code in code {
                            if code == 1 {
                                serial.write(".".as_bytes()).unwrap();
                            } else if code == 2 {
                                serial.write("-".as_bytes()).unwrap();
                            }
                        }
                        serial.write(" ".as_bytes()).unwrap();
                    }
                    Code::Space => {
                        serial.write("  ".as_bytes()).unwrap();
                    }
                    Code::Error => {
                        serial.write("%".as_bytes()).unwrap();
                    }
                    Code::None => {}
                }
            }

            serial.write(&[b'\n'; 2]).unwrap();

            loop {
                blink_codes(&mut internal_led, &mut delay, &codes)
            }
        }
    }
}

fn test_lights(lights: &mut [&mut DynPin; 6], delay: &mut Delay) {
    for light in lights.iter_mut() {
        light.set_high().unwrap();
        delay.delay_ms(150)
    }

    for light in lights {
        light.set_low().unwrap();
        delay.delay_ms(150)
    }
}
