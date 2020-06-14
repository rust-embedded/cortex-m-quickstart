//! Debugging a crash (exception)
//!
//! Most crash conditions trigger a hard fault exception, whose handler is defined via
//! `exception!(HardFault, ..)`. The `HardFault` handler has access to the exception frame, a
//! snapshot of the CPU registers at the moment of the exception.
//!
//! This program crashes and the `HardFault` handler prints to the console the contents of the
//! `ExceptionFrame` and then triggers a breakpoint. From that breakpoint one can see the backtrace
//! that led to the exception.
//!
//! ``` text
//! (gdb) continue
//! Program received signal SIGTRAP, Trace/breakpoint trap.
//! __bkpt () at asm/bkpt.s:3
//! 3         bkpt
//!
//! (gdb) backtrace
//! #0  __bkpt () at asm/bkpt.s:3
//! #1  0x080030b4 in cortex_m::asm::bkpt () at $$/cortex-m-0.5.0/src/asm.rs:19
//! #2  rust_begin_unwind (args=..., file=..., line=99, col=5) at $$/panic-semihosting-0.2.0/src/lib.rs:87
//! #3  0x08001d06 in core::panicking::panic_fmt () at libcore/panicking.rs:71
//! #4  0x080004a6 in crash::hard_fault (ef=0x20004fa0) at examples/crash.rs:99
//! #5  0x08000548 in UserHardFault (ef=0x20004fa0) at <exception macros>:10
//! #6  0x0800093a in HardFault () at asm.s:5
//! Backtrace stopped: previous frame identical to this frame (corrupt stack?)
//! ```
//!
//! In the console output one will find the state of the Program Counter (PC) register at the time
//! of the exception.
//!
//! ``` text
//! panicked at 'HardFault at ExceptionFrame {
//!     r0: 0x2fffffff,
//!     r1: 0x2fffffff,
//!     r2: 0x080051d4,
//!     r3: 0x080051d4,
//!     r12: 0x20000000,
//!     lr: 0x08000435,
//!     pc: 0x08000ab6,
//!     xpsr: 0x61000000
//! }', examples/crash.rs:106:5
//! ```
//!
//! This register contains the address of the instruction that caused the exception. In GDB one can
//! disassemble the program around this address to observe the instruction that caused the
//! exception.
//!
//! ``` text
//! (gdb) disassemble/m 0x08000ab6
//! Dump of assembler code for function core::ptr::read_volatile:
//! 451     pub unsafe fn read_volatile<T>(src: *const T) -> T {
//!    0x08000aae <+0>:     sub     sp, #16
//!    0x08000ab0 <+2>:     mov     r1, r0
//!    0x08000ab2 <+4>:     str     r0, [sp, #8]
//!
//! 452         intrinsics::volatile_load(src)
//!    0x08000ab4 <+6>:     ldr     r0, [sp, #8]
//! -> 0x08000ab6 <+8>:     ldr     r0, [r0, #0]
//!    0x08000ab8 <+10>:    str     r0, [sp, #12]
//!    0x08000aba <+12>:    ldr     r0, [sp, #12]
//!    0x08000abc <+14>:    str     r1, [sp, #4]
//!    0x08000abe <+16>:    str     r0, [sp, #0]
//!    0x08000ac0 <+18>:    b.n     0x8000ac2 <core::ptr::read_volatile+20>
//!
//! 453     }
//!    0x08000ac2 <+20>:    ldr     r0, [sp, #0]
//!    0x08000ac4 <+22>:    add     sp, #16
//!    0x08000ac6 <+24>:    bx      lr
//!
//! End of assembler dump.
//! ```
//!
//! `ldr r0, [r0, #0]` caused the exception. This instruction tried to load (read) a 32-bit word
//! from the address stored in the register `r0`. Looking again at the contents of `ExceptionFrame`
//! we see that the `r0` contained the address `0x2FFF_FFFF` when this instruction was executed.
//!
//! ---

#![no_main]
#![no_std]

use panic_halt as _;

use core::ptr;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    unsafe {
        // read an address outside of the RAM region; this causes a HardFault exception
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }

    loop {}
}
