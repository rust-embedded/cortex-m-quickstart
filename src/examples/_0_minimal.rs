//! Minimal Cortex-M program
//!
//! All Cortex-M programs need to:
//!
//! - Contain the `#![no_main]` and `#![no_std]` attributes. Embedded programs don't use the
//! standard Rust `main` interface or the Rust standard (`std`) library.
//!
//! - Define their entry point using `main!`. The entry point doesn't need to be called `main` and
//! it doesn't need to be in the root of the crate.
//!
//! - Define their panicking behavior, i.e. what happens when `panic!` is called. The easiest way to
//! define a panicking behavior is to link to a [panic implementation crate][0]
//!
//! [0]: https://crates.io/keywords/panic-impl
//!
//! - Define the `HardFault` handler. This function is called when a hard fault exception is raised
//! by the hardware.
//!
//! - Define a default handler. This function will be used to handle all interrupts and exceptions
//! which have not been assigned a specific handler.
//!
//! - Define the device specific interrupt handlers. `interrupts!` can be used to create a generic
//! program that works for all Cortex-M devices by binding all the possible interrupt handlers to
//! the `DefaultHandler`.
//!
//! ```
//! 
//! #![no_main] // <- IMPORTANT!
//! #![no_std]
//! 
//! extern crate cortex_m;
//! #[macro_use(main, exception, interrupts)]
//! extern crate cortex_m_rt as rt;
//! extern crate panic_abort; // panicking behavior
//! 
//! use cortex_m::asm;
//! use rt::ExceptionFrame;
//! 
//! // the program entry point
//! main!(main);
//! 
//! #[inline(always)]
//! fn main() -> ! {
//!     asm::bkpt();
//! 
//!     loop {}
//! }
//! 
//! // define the default exception handler
//! exception!(DefaultHandler, deh);
//! 
//! #[inline(always)]
//! fn deh(_nr: u8) {
//!     asm::bkpt();
//! }
//! 
//! // define the hard fault handler
//! exception!(HardFault, hf);
//! 
//! #[inline(always)]
//! fn hf(_ef: &ExceptionFrame) -> ! {
//!     asm::bkpt();
//! 
//!     loop {}
//! }
//! 
//! // bind all interrupts to the default exception handler
//! interrupts!(DefaultHandler);
//! ```
// Auto-generated. Do not modify.
