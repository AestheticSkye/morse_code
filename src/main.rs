#![no_std]
#![no_main]

mod button;
mod led;
mod morse;
mod pins;
mod serial;

// Imports for initialization
use cortex_m::delay::Delay;
use embedded_hal::digital::v2::InputPin;
use panic_halt as _;
use rp_pico::{
    entry,
    hal::{self, clocks::Clock, pac, usb::UsbBus, Timer, Watchdog},
};
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

use crate::button::scan;
use crate::led::blink_codes;
use crate::morse::*;
use crate::pins::PinSet;
use crate::serial::read;

const BUFFER_LENGTH: usize = 64;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

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

    let mut pin_set = PinSet::new(
        pins.gpio25.into_push_pull_output().into(),
        pins.gpio15.into_push_pull_output().into(),
        pins.gpio14.into_push_pull_output().into(),
        pins.gpio16.into_push_pull_output().into(),
        pins.gpio17.into_push_pull_output().into(),
        pins.gpio18.into_push_pull_output().into(),
        pins.gpio13.into_pull_down_input().into(),
    );

    let mut initialised = false;

    loop {
        // No clue why this has to be done, but serial wont work without it
        if !initialised && timer.get_counter().ticks() >= 2_000_000 {
            initialised = true;
            serial.write("Hello World!\r\n".as_bytes()).unwrap();
        }

        usb_dev.poll(&mut [&mut serial]);

        if initialised {
            let mut current_on = 0;

            serial
                .write(
                    "Press button to select serial mode.\nHold button to select button mode."
                        .as_bytes(),
                )
                .unwrap();

            loop {
                if pin_set.button.is_high().unwrap() {
                    current_on += 1
                } else if current_on > 300 {
                    run_serial(&mut pin_set, &mut delay, &mut serial, &mut usb_dev)
                } else if current_on > 1 {
                    run_button(&mut pin_set, &mut delay, &mut serial);
                }
                delay.delay_ms(1)
            }
        }
    }
}

fn run_button(pin_set: &mut PinSet, delay: &mut Delay, serial: &mut SerialPort<UsbBus>) {
    while pin_set.button.is_low().unwrap() {}

    let codes = scan(pin_set, delay, serial);

    serial.write(&[b'\n', b'\r']).unwrap();

    serial.write(codes_to_string(&codes).as_bytes()).unwrap();

    delay.delay_us(1);

    serial.write(&[b'\n', b'\r']).unwrap();

    loop {
        blink_codes(&mut pin_set.internal_led, delay, &codes)
    }
}

fn run_serial(
    pin_set: &mut PinSet,
    delay: &mut Delay,
    serial: &mut SerialPort<UsbBus>,
    usb_dev: &mut UsbDevice<UsbBus>,
) {
    serial
        .write("Please enter the text you wish to encode into morse.\r\n".as_bytes())
        .unwrap();

    let converted_text = read(usb_dev, serial);

    let codes = string_to_codes(&converted_text);

    serial.write(to_marks(&codes).as_bytes()).unwrap();

    // carriage return and line feed dont get written without delay ¯\_(ツ)_/¯
    delay.delay_us(1);

    serial.write(&[b'\n', b'\r', b'\n', b'\r']).unwrap();

    loop {
        blink_codes(&mut pin_set.internal_led, delay, &codes)
    }
}
