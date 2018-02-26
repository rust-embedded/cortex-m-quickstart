//! Defining the panic handler
//!
//! The panic handler can be defined through the `panic_fmt` [language item][1].
//! Make sure that the "abort-on-panic" feature of the cortex-m-rt crate is
//! disabled to avoid redefining the language item.
//!
//! [1]: https://doc.rust-lang.org/unstable-book/language-features/lang-items.html
//!
//! ---
//!
//! ```
//! 
//! #![feature(core_intrinsics)]
//! #![feature(lang_items)]
//! #![feature(used)]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! extern crate cortex_m_rt;
//! extern crate cortex_m_semihosting;
//! 
//! use core::fmt::Write;
//! use core::intrinsics;
//! 
//! use cortex_m::asm;
//! use cortex_m_semihosting::hio;
//! 
//! fn main() {
//!     panic!("Oops");
//! }
//! 
//! #[lang = "panic_fmt"]
//! #[no_mangle]
//! pub unsafe extern "C" fn rust_begin_unwind(
//!     args: core::fmt::Arguments,
//!     file: &'static str,
//!     line: u32,
//!     col: u32,
//! ) -> ! {
//!     if let Ok(mut stdout) = hio::hstdout() {
//!         write!(stdout, "panicked at '")
//!             .and_then(|_| {
//!                 stdout
//!                     .write_fmt(args)
//!                     .and_then(|_| writeln!(stdout, "', {}:{}:{}", file, line, col))
//!             })
//!             .ok();
//!     }
//! 
//!     intrinsics::abort()
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
