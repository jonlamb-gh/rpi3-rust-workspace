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
        phy_width: 240,
        phy_height: 240,

        virt_width: 240,
        virt_height: 240,

        x_offset: 0,
        y_offset: 0,
    };

    let mut display = Display::new(Some(fb_cfg), &mut mbox).unwrap();

    let bar_graph = BarGraph::new();

    loop {
        display.clear_screen(&mut mbox);

        bar_graph.test_draw(&mut display);
    }
}
