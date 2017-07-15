# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/japaric/cortex-m-quickstart/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.8...v0.2.0
[v0.1.8]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.7...v0.1.8
[v0.1.7]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.6...v0.1.7
[v0.1.6]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.5...v0.1.6
[v0.1.5]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.4...v0.1.5
[v0.1.4]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/cortex-m-quickstart/compare/v0.1.0...v0.1.1
