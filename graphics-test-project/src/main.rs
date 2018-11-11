#![no_std]
#![no_main]
#![feature(core_intrinsics, asm)]

extern crate bcm2837;
extern crate cortex_a;
extern crate display;
extern crate embedded_graphics;
extern crate mailbox;
extern crate rgb;

#[macro_use]
extern crate raspi3_boot;

mod panic_handler;
mod serial;
mod graphics;

use bcm2837::mbox::MBOX;
use bcm2837::uart0::UART0;
use core::fmt::Write;
use display::Display;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use mailbox::msg::framebuffer::FramebufferCmd;
use mailbox::Mailbox;

use graphics::bar_graph::BarGraph;
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

    let fb_cfg = FramebufferCmd {
        phy_width: 800,
        phy_height: 480,

        virt_width: 800,
        virt_height: 480,

        x_offset: 0,
        y_offset: 0,
    };

    let mut display = Display::new(Some(fb_cfg), &mut mbox).unwrap();

    loop {
        display.clear_screen(&mut mbox);

        let w: i32 = display.width() as _;
        let h: i32 = display.height() as _;

        display.draw(
            Circle::new(Coord::new(w / 2, h / 2), (h / 2) as u32 - 20)
                .with_stroke(Some((0xFF, 0x00, 0x00).into()))
                .with_fill(Some((0xFF, 0xFF, 0x00).into()))
                .into_iter(),
        );
    }
}
