#![no_std]
#![no_main]
#![feature(asm)]

extern crate bcm2837;
extern crate cortex_a;
extern crate mailbox;

#[macro_use]
extern crate raspi3_boot;

mod serial;

use bcm2837::mbox::MBOX;
use bcm2837::uart0::UART0;
use mailbox::Mailbox;
use serial::Serial;

entry!(kernel_entry);

fn kernel_entry() -> ! {
    let mut mbox = Mailbox::new(MBOX::new());
    let serial = Serial::new(UART0::new());

    // set up serial console
    if serial.init(&mut mbox).is_err() {
        // If UART fails, abort early
        loop {
            cortex_a::asm::wfe();
        }
    }

    serial.puts("Hello World\n");

    // TODO
    loop {}
}
