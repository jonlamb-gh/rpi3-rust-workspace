#![no_std]
#![no_main]
#![feature(asm)]

extern crate bcm2837;
extern crate cortex_a;
extern crate embedded_graphics;
extern crate mailbox;

#[macro_use]
extern crate raspi3_boot;

mod display;
mod serial;

use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};

use bcm2837::mbox::MBOX;
use bcm2837::uart0::UART0;
use core::fmt::Write;
use display::Display;
use mailbox::msg::framebuffer::FramebufferCmd;
use mailbox::msg::Resp;
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
        phy_width: 240,
        phy_height: 240,

        virt_width: 240,
        virt_height: 240,

        x_offset: 0,
        y_offset: 0,
    };

    writeln!(serial, "cmd = {:#?}", cmd);

    let resp = mbox.call(channel::PROP, &cmd);

    writeln!(serial, "resp = {:#?}", resp);

    if let Ok(resp) = resp {
        if let Resp::FramebufferResp(mut fb_resp) = resp {
            let mut display = Display::new(fb_resp);
            render_display(&mut display);
        }
    }

    // TODO
    loop {}
}

fn render_display(display: &mut Display) {
    // Outline
    display.draw(
        Circle::new(Coord::new(64, 64), 64)
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    // Clock hands
    display.draw(
        Line::new(Coord::new(64, 64), Coord::new(0, 64))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );
    display.draw(
        Line::new(Coord::new(64, 64), Coord::new(80, 80))
            .with_stroke(Some(1u8.into()))
            .into_iter(),
    );

    display.draw(
        Font6x8::render_str("Hello World!")
            .with_stroke(Some(1u8.into()))
            .translate(Coord::new(5, 50))
            .into_iter(),
    );
}
