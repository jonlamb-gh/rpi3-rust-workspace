#![no_std]

const MMIO_BASE: u32 = 0x3F00_0000;

pub mod bsc0;
pub mod bsc1;
pub mod bsc2;
pub mod dma;
pub mod gpio;
pub mod mbox;
pub mod rng;
pub mod spi0;
pub mod sys_timer;
pub mod uart0;
pub mod uart1;
