#![no_std]
#![no_main]
#![feature(core_intrinsics, asm)]

extern crate bcm2837;
extern crate cortex_a;
extern crate display;
extern crate embedded_graphics;
extern crate gui;
extern crate mailbox;
extern crate rgb;

#[macro_use]
extern crate raspi3_boot;

mod panic_handler;
mod serial;

use bcm2837::mbox::MBOX;
use bcm2837::uart0::UART0;
use core::fmt::Write;
use display::{Display, ObjectDrawing};
use embedded_graphics::coord::Coord;
use mailbox::msg::framebuffer::FramebufferCmd;
use mailbox::Mailbox;
use rgb::RGB8;

use gui::{BarGraph, BarGraphConfig, CircleDigit, CircleDigitConfig};
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

    let bar_graph_config = BarGraphConfig {
        top_left: Coord::new(100, 50),
        bottom_right: Coord::new(150, 250),
        background_color: RGB8::new(0xF0, 0x0F, 0xCF),
        fill_color: RGB8::new(0x00, 0xAF, 0xCF),
        text_color: RGB8::new(0xFF, 0xFF, 0xFF),
        stroke_color: RGB8::new(0xFF, 0xFF, 0xFF),
        stroke_width: 2,
    };

    let mut bar_graph = BarGraph::new(bar_graph_config);

    let mut circle_digit = CircleDigit::new(CircleDigitConfig {
        center: Coord::new(300, 200),
        radius: 30,
        fill: true,
        text_color: RGB8::new(0xFF, 0xFF, 0xFF),
        background_fill_color: RGB8::new(0xAF, 0xAF, 0x00),
        stroke_color: RGB8::new(0xFF, 0xFF, 0xFF),
        stroke_width: 2,
    });

    loop {
        display.clear_screen(&mut mbox);

        // QEMU doesn't seem to clear the display/framebuffer?
        /*
        use embedded_graphics::prelude::*;
        use embedded_graphics::primitives::Rect;
        display.draw(
            Rect::new(
                Coord::new(99, 9), Coord::new(151, 231))
                .with_fill(Some((0x00, 0x00, 0x00).into()))
                .into_iter(),
        );
        */

        bar_graph.set_value(0.90);
        bar_graph.draw_object(&mut display);

        circle_digit.set_value(4);
        circle_digit.draw_object(&mut display);

        loop {}
    }
}
