//! Sends "Hello, world!" through the ITM port 0
//!
//! **IMPORTANT** Not all Cortex-M chips support ITM. You'll have to connect the
//! microcontroller's SWO pin to the SWD interface. Note that some development
//! boards don't provide this option.
//!
//! ITM is much faster than semihosting. Like 4 orders of magnitude or so.
//!
//! You'll need [`itmdump`] to receive the message on the host plus you'll need
//! to uncomment the `monitor` commands in the `.gdbinit` file.
//!
//! [`itmdump`]: https://docs.rs/itm/0.1.1/itm/
//!
//! ---

#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::{asm, Peripherals};

fn main() {
    let p = Peripherals::take().unwrap();
    let mut itm = p.ITM;

    iprintln!(&mut itm.stim[0], "Hello, world!");
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
