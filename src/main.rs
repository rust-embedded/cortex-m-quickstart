#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt;

// TODO pick a panicking behavior
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // requires ITM support
// extern crate panic_semihosting; // requires a debugger

entry!(main);

fn main() -> ! {
    loop {
        // TODO your code goes here
    }
}
