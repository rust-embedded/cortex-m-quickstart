//! Overriding an exception handler
//!
//! You can override an exception handler using the [`exception!`][1] macro.
//!
//! [1]: https://docs.rs/cortex-m-rt/0.3.2/cortex_m_rt/macro.exception.html
//!
//! The default exception handler can be overridden using the
//! [`default_handler!`][2] macro
//!
//! [2]: https://docs.rs/cortex-m-rt/0.3.2/cortex_m_rt/macro.default_handler.html
//!
//! ---
//!
//! ```
//! 
//! #![feature(used)]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! #[macro_use(exception)]
//! extern crate cortex_m_rt;
//! 
//! use core::ptr;
//! 
//! use cortex_m::asm;
//! 
//! fn main() {
//!     unsafe {
//!         // Invalid memory access
//!         ptr::read_volatile(0x2FFF_FFFF as *const u32);
//!     }
//! }
//! 
//! exception!(HARD_FAULT, handler);
//! 
//! fn handler() {
//!     // You'll hit this breakpoint rather than the one in cortex-m-rt
//!     asm::bkpt()
//! }
//! 
//! // As we are not using interrupts, we just register a dummy catch all handler
//! #[allow(dead_code)]
//! #[used]
//! #[link_section = ".vector_table.interrupts"]
//! static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];
//! 
//! extern "C" fn default_handler() {
//!     asm::bkpt();
//! }
//! ```
// Auto-generated. Do not modify.
