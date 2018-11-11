#![no_std]
#![no_main]
#![feature(core_intrinsics, asm)]

extern crate bcm2837;
extern crate cortex_a;
extern crate display;
extern crate embedded_graphics;
extern crate heapless;
extern crate mailbox;
extern crate rgb;

#[macro_use]
extern crate raspi3_boot;

mod graphics;
mod panic_handler;
mod serial;

use bcm2837::mbox::MBOX;
use bcm2837::uart0::UART0;
use core::fmt::Write;
use display::Display;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use mailbox::msg::framebuffer::FramebufferCmd;
use mailbox::Mailbox;
use rgb::RGB8;

use graphics::bar_graph::{BarGraph, Config as BarGraphConfig};
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
        //bar_graph.test_draw(&mut display);

        //display.draw(bar_graph.test_obj());

        display.draw(bar_graph.into_iter());

        loop {}
    }
}
