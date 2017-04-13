# `cortex-m-quickstart`

> Quickstart template to develop bare metal applications for Cortex-M
> microcontrollers

## Usage

> **NOTE** The `--template` feature has been removed from Cargo recently. This
> command temporarily rollback to an older Cargo version to run the `new`
> command:

```
$ cargo +nightly-2017-04-01 new stm32f100xx --template https://github.com/japaric/cortex-m-quickstart
```

Where `stm32f100xx` is the name of the microcontroller family you are
targeting.

## Supported microcontroller families

- nrf51
- stm32f100xx
- stm32f103xx
- stm32f30x

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
