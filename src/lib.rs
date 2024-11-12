#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(abi_msp430_interrupt)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "msp430g2x01")]
pub mod msp430g2x01;

#[cfg(feature = "msp430g2x02")]
pub mod msp430g2x02;

#[cfg(feature = "msp430g2x03")]
pub mod msp430g2x03;

#[cfg(feature = "msp430g2x10")]
pub mod msp430g2x10;

#[cfg(feature = "msp430g2x30")]
pub mod msp430g2x30;

#[cfg(feature = "msp430g2x11")]
pub mod msp430g2x11;

#[cfg(feature = "msp430g2x21")]
pub mod msp430g2x21;

#[cfg(feature = "msp430g2x31")]
pub mod msp430g2x31;

#[cfg(feature = "msp430g2x12")]
pub mod msp430g2x12;

#[cfg(feature = "msp430g2x32")]
pub mod msp430g2x32;

#[cfg(feature = "msp430g2x52")]
pub mod msp430g2x52;

#[cfg(feature = "msp430g2x13")]
pub mod msp430g2x13;

#[cfg(feature = "msp430g2x33")]
pub mod msp430g2x33;

#[cfg(feature = "msp430g2x53")]
pub mod msp430g2x53;

#[cfg(feature = "msp430g2x44")]
pub mod msp430g2x44;

#[cfg(feature = "msp430g2x55")]
pub mod msp430g2x55;
