//! Overriding an exception
//!
//! **NOTE** You have to disable the `cortex-m-rt` crate's "exceptions" feature
//! to make this work.
//!
//! ```
//! 
//! #![feature(used)]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! 
//! use core::ptr;
//! 
//! use cortex_m::{asm, exception};
//! 
//! fn main() {
//!     unsafe {
//!         // Invalid memory access
//!         ptr::read_volatile(0x2FFF_FFFF as *const u32);
//!     }
//! }
//! 
//! extern "C" fn hard_fault(_: exception::HardFault) {
//!     // You'll hit this breakpoint rather than the one in cortex-m-rt
//!     asm::bkpt()
//! }
//! 
//! // When the "exceptions" feature is disabled, you'll have to provide this symbol
//! #[allow(dead_code)]
//! #[used]
//! #[link_section = ".rodata.exceptions"]
//! static EXCEPTIONS: exception::Handlers = exception::Handlers {
//!     // This is the exception handler override
//!     hard_fault: hard_fault,
//!     ..exception::DEFAULT_HANDLERS
//! };
//! 
//! // As we are not using interrupts, we just register a dummy catch all handler
//! #[allow(dead_code)]
//! #[used]
//! #[link_section = ".rodata.interrupts"]
//! static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];
//! 
//! extern "C" fn default_handler() {
//!     asm::bkpt();
//! }
//! ```
// Auto-generated. Do not modify.
