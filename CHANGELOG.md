# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.3.3] - 2018-08-07

### Changed

- Stopped recommending LLD as it requires an unstable feature.

### Fixed

- The allocator example. It now uses the `#[alloc_error_handler]` attribute
  instead of the unstable `oom` lang item.

## [v0.3.2] - 2018-06-19

### Fixed

- Bumped the panic-semihosting dependency to fix some examples when compiling with latest nightly.

## [v0.3.1] - 2018-05-13

- Document the standard `main` interface issue in the troubleshooting guide.

## [v0.3.0] - 2018-05-12

### Changed

- [breaking-change] `arm-none-eabi-gcc` is now a mandatory dependency as it's required by the
  `cortex-m-rt` dependency and also the default linker.

- Bumped the `cortex-m` and `cortex-m-rt` dependencies to v0.5.0. Updated all the examples to match
  the new `cortex-m-rt` API.

- Updated the `allocator` example to compile on a recent nightly.

- Set the number of codegen-units to 1 when compiling in release mode. This produces smaller and
  faster binaries.

### Removed

- Removed `opt-level = "s"` from `profile.release`. This flag is still unstable.

## [v0.2.7] - 2018-04-24

### Changed

- Bumped the dependency of `cortex-m-rt` to v0.4.0.

## [v0.2.6] - 2018-04-09

### Changed

- The documentation to instruct the user to use Cargo instead of Xargo

## [v0.2.5] - 2018-02-26

### Added

- Comments to Cargo.toml and Xargo.toml to make it easier to try the examples.

### Fixed

- The `allocator` example to use the `#[global_allocator]` feature.

## [v0.2.4] - 2018-01-26

### Changed

- Disable ThinLTO which causes extreme binary size bloat. See rust-lang/rust#47770 for details.

## [v0.2.3] - 2018-01-20

### Changed

- Tweaked docs. Instruction steps are now numbered.

### Removed

- The `CARGO_INCREMENTAL=1` workaround has been removed since it's now controlled via Cargo.toml and
  we have the setting disabled in the template.

## [v0.2.2] - 2018-01-17

### Added

- Troubleshooting documentation: how to workaround the "Ignoring packet error, continuing..." GDB
  error.

### Changed

- Disabled incremental compilation and parallel codegen on the dev profile to reduce the changes of
  running into rust-lang/rust#47074.

- Bumped the version of the `cortex-m-rt` dependency to v0.3.12.

## [v0.2.1] - 2017-07-14

### Added

- Troubleshooting documentation: how to fix the error of overwriting the
  `.cargo/config` file when you meant to append text to it.

### Changed

- Xargo.toml: Changed the source of the `compiler-builtins` crate from git to
  the `rust-src` component.

- Expanded the `device` example to do some I/O.

## [v0.2.0] - 2017-07-07

### Changed

- [breaking-change] Bumped the cortex-m and cortex-m-rt versions to v0.3.0.

## [v0.1.8] - 2017-05-30

### Changed

- Bumped the cortex-m-rt dependency to v0.2.3, and documented the `_stext`
  symbol (see memory.x).

## [v0.1.7] - 2017-05-27

### Added

- Documentation and an example about how to use the heap and a dynamic memory
  allocator.

### Changed

- Bumped the `cortex-m-rt` dependency to v0.2.2
- Bumped the `cortex-m` dependency to v0.2.7

## [v0.1.6] - 2017-05-26

### Added

- Set the default runner in .cargo/config to `arm-none-eabi-gdb`. Now `xargo
  run` will build the program and start a debug session.

## [v0.1.5] - 2017-05-16

### Added

- A warning about using CARGO_INCREMENTAL to the how to use and the
  troubleshooting sections.

## [v0.1.4] - 2017-05-13

### Added

- A dependencies section to the documentation

### Changed

- Extend troubleshooting section

## [v0.1.3] - 2017-05-13

### Added

- A troubleshooting section to the documentation

### Changed

- Bumped the cortex-m crate version to v0.2.6

## [v0.1.2] - 2017-05-07

### Fixed

- .gdbinit: jump to reset handler after loading the program.

## [v0.1.1] - 2017-04-27

### Changed

- Bumped the version of the `cortex-m-rt` dependency to v0.2.0. NOTE that the
  instantiation steps have slightly changed, the `memory.x` file changed,
  because of this.

## v0.1.0 - 2017-04-25

- Initial release

[Unreleased]: https://github.com/japaric/cortex-m-quickstart/compare/v0.3.3...HEAD
[v0.3.3]: https://github.com/japaric/cortex-m-quickstart/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/japaric/cortex-m-quickstart/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/japaric/cortex-m-quickstart/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.7...v0.3.0
[v0.2.7]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.8...v0.2.0
[v0.1.8]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.7...v0.1.8
[v0.1.7]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.6...v0.1.7
[v0.1.6]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.0...v0.1.1
