//! Debugging a crash (exception)
//!
//! The `cortex-m-rt` crate provides functionality for this through a default exception handler.
//! When an exception is hit, the default handler will trigger a breakpoint and in this debugging
//! context the stacked registers are accessible.
//!
//! In you run the example below, you'll be able to inspect the state of your program under the
//! debugger using these commands:
//!
//! ``` text
//! (gdb) continue
//! Program received signal SIGTRAP, Trace/breakpoint trap.
//! __bkpt () at asm/bkpt.s:3
//! 3         bkpt
//!
//! (gdb) finish
//! Run till exit from #0  __bkpt () at asm/bkpt.s:3
//! Note: automatically using hardware breakpoints for read-only addresses.
//! crash::hf (_ef=0x20004fa0) at examples/crash.rs:102
//! 99         asm::bkpt();
//!
//! (gdb) # Exception frame = program state during the crash
//! (gdb) print/x *_ef
//! $1 = cortex_m_rt::ExceptionFrame {
//!   r0: 0x2fffffff,
//!   r1: 0x2fffffff,
//!   r2: 0x80006b0,
//!   r3: 0x80006b0,
//!   r12: 0x20000000,
//!   lr: 0x800040f,
//!   pc: 0x800066a,
//!   xpsr: 0x61000000
//! }
//!
//! (gdb) # Where did we come from?
//! (gdb) backtrace
//! #0  crash::hf (_ef=0x20004fa0) at examples/crash.rs:102
//! #1  0x080004ac in UserHardFault (ef=0x20004fa0) at <exception macros>:9
//! #2  <signal handler called>
//! #3  0x0800066a in core::ptr::read_volatile (src=0x2fffffff) at /checkout/src/libcore/ptr.rs:452
//! #4  0x0800040e in crash::main () at examples/crash.rs:85
//! #5  0x08000456 in main () at <main macros>:3
//!
//! (gdb) # Nail down the location of the crash
//! (gdb) disassemble/m _ef.pc
//! Dump of assembler code for function core::ptr::read_volatile:
//! 451     pub unsafe fn read_volatile<T>(src: *const T) -> T {}
//!    0x08000662 <+0>:     sub     sp, #16
//!    0x08000664 <+2>:     mov     r1, r0
//!    0x08000666 <+4>:     str     r0, [sp, #8]
//!
//! 452         intrinsics::volatile_load(src)
//!    0x08000668 <+6>:     ldr     r0, [sp, #8]
//!    0x0800066a <+8>:     ldr     r0, [r0, #0]
//!    0x0800066c <+10>:    str     r0, [sp, #12]
//!    0x0800066e <+12>:    ldr     r0, [sp, #12]
//!    0x08000670 <+14>:    str     r1, [sp, #4]
//!    0x08000672 <+16>:    str     r0, [sp, #0]
//!    0x08000674 <+18>:    b.n     0x8000676 <core::ptr::read_volatile+20>
//!
//! 453     }
//!    0x08000676 <+20>:    ldr     r0, [sp, #0]
//!    0x08000678 <+22>:    add     sp, #16
//!    0x0800067a <+24>:    bx      lr
//!
//! End of assembler dump.
//! ```
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
//! extern crate panic_abort;
//! 
//! use core::ptr;
//! 
//! use cortex_m::asm;
//! use rt::ExceptionFrame;
//! 
//! main!(main);
//! 
//! #[inline(always)]
//! fn main() -> ! {
//!     unsafe {
//!         ptr::read_volatile(0x2FFF_FFFF as *const u32);
//!     }
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
