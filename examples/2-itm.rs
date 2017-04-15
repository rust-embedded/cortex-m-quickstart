//! Sends "Hello, world!" through the ITM port 0
//!
//! **NOTE** Not all Cortex-M chips support ITM. You'll have to connect your
//! microcontroller's SWO pin to the debug interface. Some development boards
//! don't provide this option.
//!
//! This is faster than using semihosting.
//!
//! You'll need [`itmdump`] to receive the message plus you'll need to enable
//! OpenOCD's ITM support in `.gdbinit`.
//!
//! [`itmdump`]: https://docs.rs/itm/0.1.1/itm/

#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate {{name}};

use cortex_m::peripheral;
use {{name}}::interrupt;

fn main() {
    cortex_m::interrupt::free(
        |cs| {
            let itm = peripheral::ITM.borrow(&cs);

            iprintln!(&itm.stim[0], "Hello, world!");
        },
    );
}

#[allow(dead_code)]
#[used]
#[link_section = ".rodata.interrupts"]
static INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
