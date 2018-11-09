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
use embedded_graphics::fonts::Font12x16;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};

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

    writeln!(serial, "Hello World").ok();

    let cmd = FramebufferCmd {
        phy_width: 240,
        phy_height: 240,

        virt_width: 240,
        virt_height: 240,

        x_offset: 0,
        y_offset: 0,
    };

    writeln!(serial, "cmd = {:#?}", cmd).ok();

    let resp = mbox.call(channel::PROP, &cmd);

    writeln!(serial, "resp = {:#?}", resp).ok();

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
    let w: i32 = display.width() as _;
    let h: i32 = display.height() as _;
    display.draw(
        Rect::new(Coord::new(0, 0), Coord::new(w - 1, h - 1))
            .with_stroke(Some(0x00_00_FF_u32.into()))
            .into_iter(),
    );

    display.draw(
        Circle::new(Coord::new(64, 64), 64)
            .with_stroke(Some(0x00_FF_00_u32.into()))
            .into_iter(),
    );

    display.draw(
        Line::new(Coord::new(64, 64), Coord::new(0, 64))
            .with_stroke(Some(0xFF_00_00_u32.into()))
            .with_stroke_width(1)
            .into_iter(),
    );

    display.draw(
        Line::new(Coord::new(64, 64), Coord::new(80, 80))
            .with_stroke(Some(0xFF_FF_FF_u32.into()))
            .with_stroke_width(1)
            .into_iter(),
    );

    display.draw(
        Font12x16::render_str("Hello World!")
            .with_stroke(Some(0xFF_FF_FF_u32.into()))
            .translate(Coord::new(5, 45))
            .into_iter(),
    );
}
