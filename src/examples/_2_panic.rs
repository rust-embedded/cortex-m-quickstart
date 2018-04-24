//! Changing the panic handler
//!
//! The easiest way to change the panic handler is to use a different [panic implementation
//! crate][0].
//!
//! [0]: https://crates.io/keywords/panic-impl
//!
//! ---
//!
//! ```
//! 
//! #![feature(used)]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! // extern crate panic_abort;
//! extern crate panic_semihosting; // reports panic messages to the host stderr using semihosting
//! 
//! use cortex_m::asm;
//! 
//! fn main() {
//!     panic!("Oops");
//! }
//! 
//! // As we are not using interrupts, we just register a dummy catch all handler
//! #[link_section = ".vector_table.interrupts"]
//! #[used]
//! static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];
//! 
//! extern "C" fn default_handler() {
//!     asm::bkpt();
//! }
//! ```
// Auto-generated. Do not modify.
