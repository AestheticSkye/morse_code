#![no_std]
#![no_main]

mod button;
mod led;
mod morse;
mod pins;
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

use crate::button::scan;
use crate::led::blink_codes;
use crate::morse::*;
use crate::pins::PinSet;
use crate::serial::read;
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::InputPin;
use rp2040_hal::{clocks::Clock, usb::UsbBus, Timer};

const BUFFER_LENGTH: usize = 64;

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

    let mut pin_set = PinSet::new(
        pins.gpio25.into_push_pull_output().into(),
        pins.gpio15.into_push_pull_output().into(),
        pins.gpio14.into_push_pull_output().into(),
        pins.gpio16.into_push_pull_output().into(),
        pins.gpio17.into_push_pull_output().into(),
        pins.gpio18.into_push_pull_output().into(),
        pins.gpio13.into_pull_down_input().into(),
    );

    loop {
        // No clue why this has to be done, but serial wont work without it
        if !initialised && timer.get_counter().ticks() >= 2_000_000 {
            initialised = true;
            serial.write("Hello World!\r\n".as_bytes()).unwrap();
        }

        usb_dev.poll(&mut [&mut serial]);

        if initialised {
            // serial
            //     .write("Please enter the text you wish to encode into morse.\r\n".as_bytes())
            //     .unwrap();
            //
            // let converted_text = read(&mut usb_dev, &mut serial);
            //
            // let codes = string_to_codes(&converted_text);
            //
            // serial.write(to_marks(&codes).as_bytes()).unwrap();
            //
            // // carriage return and line feed dont get written without delay ¯\_(ツ)_/¯
            // delay.delay_us(1);
            //
            // serial.write(&[b'\n', b'\r', b'\n', b'\r']).unwrap();
            //
            // loop {
            //     blink_codes(&mut internal_light, &mut delay, &codes)
            // }

            while pin_set.button.is_low().unwrap() {}

            let codes = scan(&mut pin_set, &mut delay, &mut serial);

            // serial.write(to_marks(&codes).as_bytes()).unwrap();

            serial.write(&[b'\n', b'\r']).unwrap();

            serial.write(codes_to_string(&codes).as_bytes()).unwrap();

            loop {
                blink_codes(&mut pin_set.internal_led, &mut delay, &codes)
            }
        }
    }
}
