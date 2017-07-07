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
//! ``` text
//! (gdb) # Exception frame = program state during the crash
//! (gdb) print/x *ef
//! $1 = cortex_m::exception::ExceptionFrame {
//!   r0 = 0x2fffffff,
//!   r1 = 0x2fffffff,
//!   r2 = 0x0,
//!   r3 = 0x0,
//!   r12 = 0x0,
//!   lr = 0x8000481,
//!   pc = 0x8000460,
//!   xpsr = 0x61000000,
//! }
//!
//! (gdb) # Where did we come from?
//! (gdb) backtrace
//! #0  cortex_m_rt::default_handler (ef=0x20004f54) at (..)
//! #1  <signal handler called>
//! #2  0x08000460 in core::ptr::read_volatile<u32> (src=0x2fffffff) at (..)
//! #3  0x08000480 in crash::main () at examples/crash.rs:68
//!
//! (gdb) # Nail down the location of the crash
//! (gdb) disassemble/m ef.pc
//! Dump of assembler code for function core::ptr::read_volatile<u32>:
//! 408     pub unsafe fn read_volatile<T>(src: *const T) -> T {
//!    0x08000454 <+0>:     sub     sp, #20
//!    0x08000456 <+2>:     mov     r1, r0
//!    0x08000458 <+4>:     str     r0, [sp, #8]
//!    0x0800045a <+6>:     ldr     r0, [sp, #8]
//!    0x0800045c <+8>:     str     r0, [sp, #12]
//!
//! 409         intrinsics::volatile_load(src)
//!    0x0800045e <+10>:    ldr     r0, [sp, #12]
//!    0x08000460 <+12>:    ldr     r0, [r0, #0]
//!    0x08000462 <+14>:    str     r0, [sp, #16]
//!    0x08000464 <+16>:    ldr     r0, [sp, #16]
//!    0x08000466 <+18>:    str     r1, [sp, #4]
//!    0x08000468 <+20>:    str     r0, [sp, #0]
//!    0x0800046a <+22>:    b.n     0x800046c <core::ptr::read_volatile<u32>+24>
//!
//! 410     }
//!    0x0800046c <+24>:    ldr     r0, [sp, #0]
//!    0x0800046e <+26>:    add     sp, #20
//!    0x08000470 <+28>:    bx      lr
//!
//! End of assembler dump.
//! ```
//!
//! ---

#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

use core::ptr;

use cortex_m::asm;

fn main() {
    // Read an invalid memory address
    unsafe {
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
