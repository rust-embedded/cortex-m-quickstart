//! Minimal Cortex-M program
//!
//! When executed this program will hit the breakpoint set in `main`.
//!
//! All Cortex-M programs need to:
//!
//! - Contain the `#![no_main]` and `#![no_std]` attributes. Embedded programs don't use the
//! standard Rust `main` interface or the Rust standard (`std`) library.
//!
//! - Define their entry point using [`entry!`] macro.
//!
//! [`entry!`]: https://docs.rs/cortex-m-rt/~0.5/cortex_m_rt/macro.entry.html
//!
//! - Define their panicking behavior, i.e. what happens when `panic!` is called. The easiest way to
//! define a panicking behavior is to link to a [panic handler crate][0]
//!
//! [0]: https://crates.io/keywords/panic-impl
//!
//! - Define the `HardFault` handler using the [`exception!`] macro. This handler (function) is
//! called when a hard fault exception is raised by the hardware.
//!
//! [`exception!`]: https://docs.rs/cortex-m-rt/~0.5/cortex_m_rt/macro..html
//!
//! - Define a default handler using the [`exception!`] macro. This function will be used to handle
//! all interrupts and exceptions which have not been assigned a specific handler.
//!
//! ```
//!
//! #![no_main] // <- IMPORTANT!
//! #![no_std]
//!
//! extern crate cortex_m;
//!
//! #[macro_use(entry, exception)]
//! extern crate cortex_m_rt as rt;
//!
//! // makes `panic!` print messages to the host stderr using semihosting
//! extern crate panic_semihosting;
//!
//! use cortex_m::asm;
//! use rt::ExceptionFrame;
//!
//! // the program entry point is ...
//! entry!(main);
//!
//! // ... this never ending function
//! fn main() -> ! {
//!     loop {
//!         asm::bkpt();
//!     }
//! }
//!
//! // define the hard fault handler
//! exception!(HardFault, hard_fault);
//!
//! fn hard_fault(ef: &ExceptionFrame) -> ! {
//!     panic!("HardFault at {:#?}", ef);
//! }
//!
//! // define the default exception handler
//! exception!(*, default_handler);
//!
//! fn default_handler(irqn: i16) {
//!     panic!("Unhandled exception (IRQn = {})", irqn);
//! }
//! ```
// Auto-generated. Do not modify.
