//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.
//!
//! The build script also sets the linker flags to tell it which link script to use.
//!
//! 该构建脚本将 `memory.x` 文件从 crate 根目录复制到一个目录中，以便链接器在构建时始终能够找到它。
//! 对于许多项目来说，这是可选的，因为链接器始终会搜索项目根目录 —— 即 `Cargo.toml` 所在的目录。
//! 但是，如果您使用的是工作区或具有更复杂的构建设置，则此构建脚本是必需的。
//! 此外，通过请求 Cargo 在 `memory.x` 更改时重新运行构建脚本，更新 `memory.x` 可确保使用新的内存设置重新构建应用程序。
//!
//! 该构建脚本还设置了链接器标志，以告诉链接器使用哪个链接脚本。

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    // 将 `memory.x` 放入我们的输出目录，并确保它在链接器的搜索路径中。
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    // 默认情况下，Cargo 会在项目中的任何文件更改时重新运行构建脚本。
    // 通过在此处指定 `memory.x`，我们确保仅在 `memory.x` 更改时重新运行构建脚本。
    println!("cargo:rerun-if-changed=memory.x");

    // Specify linker arguments.
    // 指定链接器参数。

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    // See https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    // 如果内存段地址未对齐到 0x10000，则需要 `--nmagic`，
    // 例如 `memory.x` 中的 FLASH 和 RAM 段。
    // 参见 https://github.com/rust-embedded/cortex-m-quickstart/pull/95
    println!("cargo:rustc-link-arg=--nmagic");

    // Set the linker script to the one provided by cortex-m-rt.
    // 将链接脚本设置为 cortex-m-rt 提供的链接脚本。
    println!("cargo:rustc-link-arg=-Tlink.x");
}