# `msp430g2xx`

`msp430g2xx` is the overarching Peripheral Access Crate (PAC) for using Rust
with the MSP430G2xx series of microcontrollers. The crate provides a safe API
to access peripherals and interrupt vectors 

`msp430g2xx` is automatically generated using a combination of
[`svd2rust`](https://github.com/rust-embedded/svd2rust) and
[`msp430gen`](https://github.com/pftbest/msp430_svd). See the
[msp430 section](https://docs.rs/svd2rust/latest/svd2rust/#target--msp430) of
`svd2rust` and the [README.md](https://github.com/pftbest/msp430_svd) of the
`msp430_svd`/`msp430gen` repository for details.

## Usage

In your application's `Cargo.toml`:

```toml
[dependencies.msp430g2xx]
version = "0.5.0"
features = ["rt", "$FAMILY"]
```

The `rt` feature enables interrupt vectors, and is, in practice, [required](https://github.com/rust-embedded/msp430-rt/issues/9).
See the [`rt` section](https://docs.rs/svd2rust/latest/svd2rust/#the-rt-feature)
of the `svd2rust` docs for details.

`$FAMILY` is one of the following features, which represent groups of devices
with the same exact set of peripherals:

`msp430g2x01`, `msp430g2x02`, `msp430g2x03`, `msp430g2x10`, `msp430g2x30`,
`msp430g2x11`, `msp430g2x21`, `msp430g2x31`, `msp430g2x12`, `msp430g2x32`,
`msp430g2x52`, `msp430g2x13`, `msp430g2x33`, `msp430g2x53`, `msp430g2x44`,
`msp430g2x55`.

With a given feature, the `x` represents the flash size. This has no bearing
on peripheral layout/types, or interrupt vectors, or types. Since other memory
layout details are generally not kept inside an SVD file, each device which
differs only in flash size can share the same SVD file.

For instance, an application targeting the [MSP430G2211](https://www.ti.com/product/MSP430G2211)
part, and another application targeting the [MSP430G2111](https://www.ti.com/product/MSP430G2111)
part would both enable the `msp4302x11` feature; only their [`memory.x`](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/#memory)
will differ.

At present, `$FAMILY` features are mutually exclusive. The build will fail
if none of _or_ more than one of the `$FAMILY` features is selected. `cargo`
[discourages](https://doc.rust-lang.org/cargo/reference/features.html#mutually-exclusive-features)
this, but it's commonly accepted in embedded Rust. Perhaps
[Feature Precedence](https://doc.rust-lang.org/cargo/reference/features-examples.html#feature-precedence)
could be possible in the future.
