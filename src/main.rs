/*
 * Calliope || main.rs
 * --------------------------------
 * Contains the programming logic for voice modification.
 *
 * Authors: @MaxineToTheStars <https://github.com/MaxineToTheStars>
 * ----------------------------------------------------------------
 */

// Namespace Attributes
#![no_std]
#![no_main]

// Crates
use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, Clocks, CpuClock},
    peripherals::Peripherals,
    prelude::*,
};

// Main
#[entry]
fn main() -> ! {
    // Initialize system peripherals
    let peripherals: Peripherals = Peripherals::take();

    // Initialize the system
    let system = peripherals.SYSTEM.split();

    // Initialize the internal clock
    let clocks: Clocks = ClockControl::configure(system.clock_control, CpuClock::Clock80MHz).freeze();

    // Initialize internal clock\
    loop {}
}
