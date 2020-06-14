//! Overriding an exception handler
//!
//! You can override an exception handler using the [`#[exception]`][1] attribute.
//!
//! [1]: https://rust-embedded.github.io/cortex-m-rt/0.6.1/cortex_m_rt_macros/fn.exception.html
//!
//! ---

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprint;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    loop {}
}

#[exception]
fn SysTick() {
    hprint!(".").unwrap();
}
