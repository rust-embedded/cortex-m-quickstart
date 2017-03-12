//! Sends "Hello, world!" through the ITM port 0
//!
//! You'll need [`itmdump`] to receive the message plus you'll need to enable
//! OpenOCD's ITM support in `.gdbinit`.
//!
//! [`itmdump`]: https://docs.rs/itm/0.1.1/itm/

#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate {{name}};

use cortex_m::{exception, peripheral};
use {{name}}::interrupt;

fn main() {
    cortex_m::interrupt::free(|cs| {
                                  let itm = peripheral::ITM.borrow(&cs);

                                  iprintln!(&itm.stim[0], "Hello, world!");
                              });
}

#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };
