//! Sends "Hello, world!" through the ITM port 0
//!
//! **IMPORTANT** Not all Cortex-M chips support ITM. You'll have to connect the microcontroller's
//! SWO pin to the SWD interface. Note that some development boards don't provide this option.
//!
//! ITM is much faster than semihosting. Like 4 orders of magnitude or so.
//!
//! You'll need [`itmdump`] to receive the message on the host plus you'll need to uncomment the
//! `monitor` commands in the `.gdbinit` file.
//!
//! [`itmdump`]: https://docs.rs/itm/0.2.1/itm/
//!
//! ---
//!
//! ```
//! 
//! #![no_main]
//! #![no_std]
//! 
//! #[macro_use]
//! extern crate cortex_m;
//! #[macro_use]
//! extern crate cortex_m_rt as rt;
//! extern crate panic_abort; // panicking behavior
//! 
//! use cortex_m::{asm, Peripherals};
//! use rt::ExceptionFrame;
//! 
//! main!(main);
//! 
//! #[inline(always)]
//! fn main() -> ! {
//!     let mut p = Peripherals::take().unwrap();
//!     let stim = &mut p.ITM.stim[0];
//! 
//!     iprintln!(stim, "Hello, world!");
//! 
//!     loop {}
//! }
//! 
//! exception!(DefaultHandler, dh);
//! 
//! #[inline(always)]
//! fn dh(_nr: u8) {
//!     asm::bkpt();
//! }
//! 
//! exception!(HardFault, hf);
//! 
//! #[inline(always)]
//! fn hf(_ef: &ExceptionFrame) -> ! {
//!     asm::bkpt();
//! 
//!     loop {}
//! }
//! 
//! interrupts!(DefaultHandler);
//! ```
// Auto-generated. Do not modify.
