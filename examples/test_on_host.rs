//! Conditionally compiling tests with std and our executable with no_std.
//!
//! Ensure there are no targets specified under `[build]` in `.cargo/config`
//! In order to make this work, we lose the convenience of having a default target that isn't the
//! host.
//!
//! cargo build --example test_on_host --target thumbv7m-none-eabi
//! cargo test --example test_on_host

#![cfg_attr(test, allow(unused_imports))]

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// pick a panicking behavior
#[cfg(not(test))]
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[cfg(not(test))]
#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn foo() {
    println!("tests work!");
    assert!(2 == add(1,1));
  }
}
