//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---
//!
//! ```
//!
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate cortex_m_rt as rt;
//! extern crate cortex_m_semihosting as sh;
//! extern crate panic_semihosting;
//!
//! use core::fmt::Write;
//!
//! use rt::ExceptionFrame;
//! use sh::hio;
//!
//! entry!(main);
//!
//! fn main() -> ! {
//!     let mut stdout = hio::hstdout().unwrap();
//!     writeln!(stdout, "Hello, world!").unwrap();
//!
//!     loop {}
//! }
//!
//! exception!(HardFault, hard_fault);
//!
//! fn hard_fault(ef: &ExceptionFrame) -> ! {
//!     panic!("HardFault at {:#?}", ef);
//! }
//!
//! exception!(*, default_handler);
//!
//! fn default_handler(irqn: i16) {
//!     panic!("Unhandled exception (IRQn = {})", irqn);
//! }
//! ```
// Auto-generated. Do not modify.
