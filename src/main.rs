//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

mod hid;
mod joystick;

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;

use embedded_time::fixed_point::FixedPoint;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use bsp::hal::{self};
use rp_pico as bsp;

use bsp::hal::clocks::Clock;
use bsp::hal::pac;

use crate::{hid::init_hid, joystick::SpustickReport};

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

    init_hid(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        &mut pac.RESETS,
        clocks.usb_clock,
    );

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // Move the cursor up and down every 200ms
    loop {
        delay.delay_ms(1000);

        let rep_up = SpustickReport {
            x: 100,
            y: 50,
            buttons: 0,
        };
        hid::push_mouse_movement(rep_up).ok().unwrap_or(0);

        delay.delay_ms(1000);

        let rep_down = SpustickReport {
            x: -50,
            y: -100,
            buttons: 0xf,
        };
        hid::push_mouse_movement(rep_down).ok().unwrap_or(0);
    }
}

// End of file
