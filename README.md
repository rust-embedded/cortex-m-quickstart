# `cortex-m-quickstart`

> A template for building applications for ARM Cortex-M microcontrollers

This project is developed and maintained by the [Cortex-M team][team].

# [Documentation](https://rust-embedded.github.io/cortex-m-quickstart/cortex_m_quickstart)

## Dependencies

To build embedded programs using this template you'll need:

- Nightly Rust toolchain from 2018-08-28 or newer: `rustup default nightly`
- The `cargo generate` subcommand: `cargo install cargo-generate`
- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Non-dependencies

This section list programs that are *not* required to build embedded programs
but are useful or necessary for embedded development.

To flash (put the firmware on the target device) and debug embedded programs
you'll need these additional programs:

<!-- TODO These bullets should link to the debugonomicon, which has instructions -->
<!-- on how to install these tools -->

- GDB with ARM support or LLDB, for debugging.
- QEMU with ARM support, for running embedded programs on the build machine.
- OpenOCD and similar, which make debugging possible at all.

To inspect the produced binaries you'll want [`cargo-binutils`].

[`cargo-binutils`]: https://crates.io/crates/cargo-binutils

## Usage

### First timers

#### Building

If you are already familiar with the process of building and debugging embedded
Rust programs feel free to skip this section!

To get you familiar with building and debugging embedded Rust programs we'll use
the template with its default values to build a program for the LM3S6965, a
microcontroller with a Cortex-M3 core that QEMU can emulate.

In this section we'll use a debugger (GDB or LLDB) and QEMU. Be sure to have
them installed!

1. Initialize the template

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

2. Build the example "Hello, world!" program

``` rust
// example/hello.rs
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;

use core::fmt::Write;

use sh::hio;

entry!(main);

fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Hello, world!").unwrap();

    loop {}
}
```

We'll cross compile the program for the `thumbv7m-none-eabi` target. This target
covers Cortex-M3 devices like the one we are targeting.

``` console
$ cargo build --target thumbv7m-none-eabi --example hello
```
You'll find the output binary in the following path:
`target/thumbv7m-none-eabi/debug/examples/hello`. The output is an ELF file.

If you have `file` installed you can print the file type of the output to
confirm it's an ELF file:

```
$ ( cd target/thumbv7m-none-eabi/debug/examples && file hello )
hello: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, with debug_info, not stripped
```

If you have `cargo-binutils` installed you can look at the ELF header of the
output:

> NOTE `cargo-readelf` will be available in a *future* release of binutils
> (v0.1.3)

``` console
$ cargo readelf --example hello -- --file-headers
```

``` text
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0x0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x23A3
  Start of program headers:          52 (bytes into file)
  Start of section headers:          673340 (bytes into file)
  Flags:                             0x5000200
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         21
  Section header string table index: 19
```

If you look at the bottom of the Cargo configuration file (`.cargo/config`)
you'll see that the `thumbv7m-none-eabi` target has been set as the default
compilation target. This means that you do *not* need to pass the `--target`
flag to cross compile because it's already implied. So:

``` console
$ cargo build --example hello
```

Does the same as before.

3. There's no step 3! Your build is done, but for completeness let's look at how
   to run this program on QEMU.

#### Running the program on QEMU

Execute this command and you are done.

``` console
$ qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -nographic \
      -semihosting-config enable=on,target=native \
      -kernel target/thumbv7m-none-eabi/debug/examples/hello
Hello, world!
```

The above command will block the terminal because the `hello` program never
ends! To terminate QEMU input this in your terminal: `Ctrl+A`, followed by `X`.

Let me break down that long command for you:

- `qemu-system-arm`. This is the QEMU emulator. There are a few variants of
  these QEMU binaries; this one does full *system* emulation of *ARM* machines
  -- hence the name.

- `-cpu cortex-m3`. This tells QEMU to emulate a Cortex-M3 CPU. Specifying the
  CPU model lets us catch some miscompilation errors: for example, running a
  program compiled for the Cortex-M4F, which has a hardware FPU, will make QEMU
  error during its execution.

- `-machine lm3s6965evb`. This tells QEMU to emulate the LM3S6965EVB, a
  evaluation board that contains a LM3S6965 microcontroller.

- `-nographic`. This tells QEMU to not launch its GUI.

- `-semihosting-config (..)`. This tells QEMU to enable semihosting. Semihosting
  lets the emulated device, among other things, use the host stdout, stderr and
  stdin and create files on the host.

- `-kernel $file`. This tells QEMU which binary to flash (kind of) and run on
  the emulated machine.

#### Debugging

First, we launch a QEMU instance:

``` console
$ qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -nographic \
      -semihosting-config enable=on,target=native \
      -gdb tcp::3333 \
      -S \
      -kernel target/thumbv7m-none-eabi/debug/examples/hello
```

Note that this command won't print anything to the console but will block the
terminal. We have passed two extra flags this time:

- `-gdb tcp::3333`. This tells QEMU to wait for a GDB connection on TCP
  port 3333.

- `-S`. This tells QEMU to freeze the machine at startup. Without this the
  program would have reached the end of `main` before we had a chance to launch
  the debugger!

Next, we start up LLDB in another terminal:

``` console
$ lldb target/thumbv7m-none-eabi/debug/examples/hello
```

And tell it to connect to the GDB server that QEMU created:

``` console
(lldb) gdb-remote 3333
Process 1 stopped
* thread #1, stop reason = signal SIGTRAP
    frame #0: 0x000023a2 hello`Reset at lib.rs:475
   472
   473  #[doc(hidden)]
   474  #[no_mangle]
-> 475  pub unsafe extern "C" fn Reset() -> ! {
   476      extern "C" {
   477          // This symbol will be provided by the user via the `entry!` macro
   478          fn main() -> !;
```

You'll see that the process is halted and that the program counter is pointing
to a function named `Reset`. That is the reset handler: what Cortex-M cores
execute upon booting.

This reset handler will eventually call our `main` function. Let's skip all the
way there using a breakpoint and the `continue` command:

``` console
(lldb) breakpoint set -name hello::main
Breakpoint 1: where = hello`hello::main::h9cf19a1378cbd1b8 + 2 at hello.rs:20, address = 0x000006a8

(lldb) continue
Process 1 resuming
Process 1 stopped
* thread #1, stop reason = breakpoint 1.1
    frame #0: 0x000006a8 hello`hello::main::h9cf19a1378cbd1b8 at hello.rs:20
   17   entry!(main);
   18
   19   fn main() -> ! {
-> 20       let mut stdout = hio::hstdout().unwrap();
   21       writeln!(stdout, "Hello, world!").unwrap();
   22
   23       loop {}
```

We are now close to the code that prints "Hello, world!". Let's move the program
forward using the `next` command.

``` console
(lldb) next
Process 1 stopped
* thread #1, stop reason = step over
    frame #0: 0x000006be hello`hello::main::h9cf19a1378cbd1b8 at hello.rs:21
   18
   19   fn main() -> ! {
   20       let mut stdout = hio::hstdout().unwrap();
-> 21       writeln!(stdout, "Hello, world!").unwrap();
   22
   23       loop {}
   24   }

(lldb) next
Process 1 stopped
* thread #1, stop reason = step over
    frame #0: 0x000006f6 hello`hello::main::h9cf19a1378cbd1b8 at hello.rs:23
   20       let mut stdout = hio::hstdout().unwrap();
   21       writeln!(stdout, "Hello, world!").unwrap();
   22
-> 23       loop {}
   24   }
```

At this point you should see "Hello, world!" printed on the terminal that's
running `qemu-system-arm`.

``` console
$ qemu-system-arm (..)
Hello, world!
```

That's it! You can now exit `lldb`, which will also cause QEMU to terminate.

```
(lldb) exit
Quitting LLDB will kill one or more processes. Do you really want to proceed: [Y/n] y
```

### Not first timers

#### Building

This section explains how to configure this template to build programs for some
specific Cortex-M device.

0. Before everything else, you need to know the characteristics of the target
   device:

- What's the ARM core? e.g. Cortex-M3.

- Does the ARM core include an FPU? e.g. the Cortex-M4F does.

- How much Flash memory and RAM does the target device has? e.g. 40 KB of RAM

- Where is Flash memory and RAM mapped in the address space? e.g. `0x2000_0000`
  is common for RAM.

You should be able to find this information in the data sheet and / or reference
manual of your device.

As an example, we'll use the [STM32F303VCT6] microcontroller. This device has:

[STM32F303VCT6]: https://www.st.com/en/microcontrollers/stm32f303vc.html

- a Cortex-M4F core that includes a single precision FPU

- 256 KB of Flash located at address `0x0800_0000`.

- 40 KB of RAM located at address `0x2000_0000`. (There's another RAM region but
  for simplicity we'll ignore it).

1. Initialize the template

``` console
$ cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

2. Set the default compilation target in `.cargo/config`.

For the STM32F303VCT6, we pick the `thumbv7em-none-eabihf` target as that covers
the Cortex-M4F core.

``` console
$ tail .cargo/config
```

``` toml
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```

3. Enter the memory region information into `memory.x`.

As we mentioned before the STM32F303VCT6 has 40 KB of RAM located at address
`0x2000_0000` and 256 KB of Flash memory located at address `0x0800_0000`.

``` console
$ cat memory.x
```

``` text
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

4. Build one of the examples:

``` console
$ cargo build --example hello
```

You are done! You should be able to flash and run this example on your
device.

#### Debugging

Teaching you how to flash and debug this program on *your* device is out of
scope for this document due to the sheer variety of possible hardware / software
combinations. But the steps required to flash and debug this program on the
[STM32F3DISCOVERY] using OpenOCD are provided below as a reference.

[STM32F3DISCOVERY]: https://www.st.com/en/evaluation-tools/stm32f3discovery.html

On a terminal run `openocd` to connect to the ST-LINK on the discovery board.
Run this command from the root of the template; `openocd` will pick up the
`openocd.cfg` file which indicates which interface file and target file to use.

``` console
$ cat openocd.cfg
```

``` text
source [find interface/stlink-v2-1.cfg]
source [find target/stm32f3x.cfg]
```

``` console
$ openocd
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
adapter speed: 1000 kHz
adapter_nsrst_delay: 100
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
none separate
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : Unable to match requested speed 1000 kHz, using 950 kHz
Info : clock speed 950 kHz
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 2.913879
Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

On another terminal run GDB, also from the root of the template.

``` console
$ cat openocd.gdb
```

``` text
target remote :3333

# print demangled symbols
set print asm-demangle on

# detect unhandled exceptions and hard faults
break DefaultHandler
break UserHardFault

monitor arm semihosting enable

load
```

``` console
$ arm-none-eabi-gdb -x openocd.gdb target/thumbv7em-none-eabihf/debug/examples/hello
(..)
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x21dc lma 0x8000400
Loading section .rodata, size 0x6a4 lma 0x80025e0
Start address 0x800238c, load size 11392
Transfer rate: 17 KB/sec, 3797 bytes/write.

(gdb) list
470     #[no_mangle]
471     pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
472
473     #[doc(hidden)]
474     #[no_mangle]
475     pub unsafe extern "C" fn Reset() -> ! {
476         extern "C" {
477             // This symbol will be provided by the user via the `entry!` macro
478             fn main() -> !;
479
```

The `openocd.gdb` script will connect GDB to OpenOCD and then flash the program
into the device. After that you can do a normal debugging session.

If you `continue` the program past the semihosting write operation you'll see
"Hello, world" printed on the OpenOCD console.

``` console
(gdb) continue
```

``` console
$ openocd
(..)
Hello, world!
```

## Next steps

> TODO point the reader to embedded-hal, awesome-embedded-rust, etc.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
