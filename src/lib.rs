//! A template for building applications for ARM Cortex-M microcontrollers
//!
//! # Usage
//!
//! - Clone this crate
//!
//! ``` text
//! $ cargo clone cortex-m-quickstart && cd $_
//! ```
//!
//! - Change the crate name, author and version
//!
//! ``` text
//! $ edit Cargo.toml && head $_
//! [package]
//! authors = ["Jorge Aparicio <jorge@japaric.io>"]
//! name = "demo"
//! version = "0.1.0"
//! ```
//!
//! - Specify the memory layout of the target device
//!
//! ``` text
//! $ edit memory.x && cat $_
//! MEMORY
//! {
//!   /* NOTE K = KiBi = 1024 bytes */
//!   FLASH : ORIGIN = 0x08000000, LENGTH = 256K
//!   RAM : ORIGIN = 0x20000000, LENGTH = 40K
//! }
//! ```
//!
//! - Optionally, set a default build target
//!
//! ``` text
//! $ cat >>.cargo/config <<'EOF'
//! [build]
//! target = "thumbv7em-none-eabihf"
//! EOF
//! ```
//!
//! - Very likely, depend on a device or a BSP (Board Support Package) crate.
//!
//! ``` text
//! # add a device crate, or
//! $ cargo add stm32f30x
//!
//! # add a BSP crate
//! $ cargo add f3
//! ```
//!
//! - Write the application or start from one of the examples
//!
//! ``` text
//! $ rm -r src/* && cp examples/hello.rs src/main.rs
//! ```
//!
//! - Build the application
//!
//! ``` text
//! # if not installed
//! $ cargo install xargo
//!
//! # NOTE this command requires `arm-none-eabi-ld` to be in $PATH
//! $ xargo build --release
//!
//! $ arm-none-eabi-readelf -A target/thumbv7em-none-eabihf/release/demo
//! Attribute Section: aeabi
//! File Attributes
//!   Tag_conformance: "2.09"
//!   Tag_CPU_arch: v7E-M
//!   Tag_CPU_arch_profile: Microcontroller
//!   Tag_THUMB_ISA_use: Thumb-2
//!   Tag_FP_arch: VFPv4-D16
//!   Tag_ABI_PCS_GOT_use: direct
//!   Tag_ABI_FP_denormal: Needed
//!   Tag_ABI_FP_exceptions: Needed
//!   Tag_ABI_FP_number_model: IEEE 754
//!   Tag_ABI_align_needed: 8-byte
//!   Tag_ABI_align_preserved: 8-byte, except leaf SP
//!   Tag_ABI_HardFP_use: SP only
//!   Tag_ABI_VFP_args: VFP registers
//!   Tag_ABI_optimization_goals: Aggressive Speed
//!   Tag_CPU_unaligned_access: v6
//!   Tag_FP_HP_extension: Allowed
//!   Tag_ABI_FP_16bit_format: IEEE 754
//! ```
//!
//! # Examples
//!
//! Check the [examples module](./examples/index.html)

#![no_std]

pub mod examples;
