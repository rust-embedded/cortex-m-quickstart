//! Prints "Hello, world!" on the OpenOCD console using semihosting
//!
//! ---
//!
//! ```
//! 
//! #![no_main]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! #[macro_use]
//! extern crate cortex_m_rt as rt;
//! extern crate cortex_m_semihosting as sh;
//! extern crate panic_abort;
//! 
//! use core::fmt::Write;
//! 
//! use cortex_m::asm;
//! use rt::ExceptionFrame;
//! use sh::hio;
//! 
//! main!(main);
//! 
//! fn main() -> ! {
//!     let mut stdout = hio::hstdout().unwrap();
//!     writeln!(stdout, "Hello, world!").unwrap();
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
//! // As we are not using interrupts, we just bind them all to the `DefaultHandler` exception handler
//! interrupts!(DefaultHandler);
//! ```
// Auto-generated. Do not modify.
