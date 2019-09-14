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

mod clock;
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

use clock::{Clock, Config as ClockConfig};
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

    let clock = Clock::new(ClockConfig {
        center: Coord::new(display.width() as i32 / 2, display.height() as i32 / 2),
        radius: (display.height() / 2) - 1,
        outline_stroke_width: 4,
        outline_color: RGB8::new(0xFF, 0xFF, 0xFF),
        /*
        sec_cd_config: CircleDigitConfig {
            center: Coord::new(300, 200),
            radius: 20,
            fill: true,
            text_color: RGB8::new(0xFF, 0xFF, 0xFF),
            background_fill_color: RGB8::new(0x0F, 0xAF, 0xF0),
            stroke_color: RGB8::new(0xFF, 0xFF, 0xFF),
            stroke_width: 2,
        },
        */
    });

    loop {
        display.clear_screen(&mut mbox);

        clock.draw_object(&mut display);

        // TESTING
        loop {}
    }
}