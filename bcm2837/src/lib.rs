#![no_std]

#[macro_use]
extern crate register;

const MMIO_BASE: u32 = 0x3F00_0000;

pub mod gpio;
pub mod mbox;
