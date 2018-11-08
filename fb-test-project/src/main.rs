#![no_std]
#![no_main]
#![feature(asm)]

extern crate bcm2837;
extern crate cortex_a;
extern crate mailbox;

#[macro_use]
extern crate raspi3_boot;

entry!(kernel_entry);

fn kernel_entry() -> ! {
    // TODO
    loop {}
}
