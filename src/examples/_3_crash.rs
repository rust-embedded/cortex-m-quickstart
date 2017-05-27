//! Debugging a crash (exception)
//!
//! The `cortex-m-rt` crate provides functionality for this through a default
//! exception handler. When an exception is hit, the default handler will
//! trigger a breakpoint and in this debugging context the stacked registers
//! are accessible.
//!
//! In you run the example below, you'll be able to inspect the state of your
//! program under the debugger using these commands:
//!
//! ```
//! (gdb) # Stacked registers = program state during the crash
//! (gdb) print/x *_sr
//! $1 = cortex_m::exception::StackedRegisters {
//!   r0 = 0x2fffffff,
//!   r1 = 0x2fffffff,
//!   r2 = 0x0,
//!   r3 = 0x0,
//!   r12 = 0x0,
//!   lr = 0x8000443,
//!   pc = 0x8000190,
//!   xpsr = 0x61000200,
//! }
//!
//! (gdb) # What exception was triggered?
//! (gdb) print _e
//! $2 = cortex_m::exception::Exception::HardFault
//!
//! (gdb) # Where did we come from?
//! (gdb) backtrace
//! ```
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
//! use cortex_m::asm;
//! 
//! fn main() {
//!     // Read an invalid memory address
//!     unsafe {
//!         ptr::read_volatile(0x2FFF_FFFF as *const u32);
//!     }
//! }
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
