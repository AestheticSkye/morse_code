// Imports for initialization
use cortex_m::delay::Delay;
use panic_halt as _;
use rp_pico::hal::{self, clocks::Clock, pac, usb::UsbBus, Timer, Watchdog};
use usb_device::{class_prelude::UsbBusAllocator, prelude::*};
use usbd_serial::SerialPort;

use crate::pins::PinSet;

pub fn initialize_system() -> (UsbBusAllocator<UsbBus>, Delay, Timer, PinSet) {
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

	let delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

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

	let pin_set = PinSet::new(
		pins.gpio25.into_push_pull_output().into(),
		pins.gpio15.into_push_pull_output().into(),
		pins.gpio14.into_push_pull_output().into(),
		pins.gpio16.into_push_pull_output().into(),
		pins.gpio17.into_push_pull_output().into(),
		pins.gpio18.into_push_pull_output().into(),
		pins.gpio13.into_pull_down_input().into(),
	);

	(usb_bus, delay, timer, pin_set)
}

pub fn initialize_usb(
	usb_bus: &UsbBusAllocator<UsbBus>,
) -> (SerialPort<UsbBus>, UsbDevice<UsbBus>) {
	// Set up the USB Communications Class Device driver
	let serial = SerialPort::new(usb_bus);

	// Create a USB device with a fake VID and PID
	let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x16c0, 0x27dd))
		.manufacturer("Fake company")
		.product("Serial port")
		.serial_number("TEST")
		.device_class(2) // from: https://www.usb.org/defined-class-codes
		.build();

	(serial, usb_dev)
}
