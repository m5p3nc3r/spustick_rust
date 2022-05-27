//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod hid;
mod input;
mod report;

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;

use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use bsp::hal;
use rp_pico as bsp;

use bsp::hal::clocks::Clock;
use bsp::hal::pac;

use crate::{
    hid::{init_hid, USBDeviceDetails},
    input::JoystickInput,
    report::SpustickReport,
};

static DETAILS: USBDeviceDetails = USBDeviceDetails {
    manufacturer: "Apptabulous",
    product: "Spustick",
    serial_number: "00000000",
    device_class: 0,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    init_hid(
        &DETAILS,
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        &mut pac.RESETS,
        clocks.usb_clock,
    );

    let input = JoystickInput::new(
        pins.gpio2.into_pull_up_input(),
        pins.gpio3.into_pull_up_input(),
        pins.gpio4.into_pull_up_input(),
        pins.gpio5.into_pull_up_input(),
        pins.gpio6.into_pull_up_input(),
        pins.gpio7.into_pull_up_input(),
    );

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let mut report: SpustickReport = Default::default();

    loop {
        // Read input, storing the result in the HID report
        input.process(&mut report);
        // Push the report
        hid::send_report(report).unwrap_or(0);
        // Delay until we read the input again
        delay.delay_ms(1000 / 30);
    }
}

// End of file
