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
use core::fmt::Write;
use mailbox::msg::framebuffer::FramebufferCmd;
use mailbox::{channel, Mailbox};
use serial::Serial;

entry!(kernel_entry);

fn kernel_entry() -> ! {
    let mut mbox = Mailbox::new(MBOX::new());
    let mut serial = Serial::new(UART0::new());

    // set up serial console
    if serial.init(&mut mbox).is_err() {
        // If UART fails, abort early
        loop {
            cortex_a::asm::wfe();
        }
    }

    writeln!(serial, "Hello World");

    let cmd = FramebufferCmd {
        phy_width: 680,
        phy_height: 480,

        virt_width: 680,
        virt_height: 480,

        x_offset: 0,
        y_offset: 0,
    };

    writeln!(serial, "cmd = {:#?}", cmd);

    let resp = mbox.call(channel::PROP, &cmd);

    writeln!(serial, "resp = {:#?}", resp);

    // TODO
    loop {}
}
