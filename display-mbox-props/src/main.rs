#![no_std]
#![no_main]
#![feature(asm)]

extern crate bcm2837;
extern crate cortex_a;
extern crate display;
extern crate embedded_graphics;
extern crate mailbox;
extern crate rgb;

#[macro_use]
extern crate raspi3_boot;

mod serial;

use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font12x16;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rect};

use bcm2837::mbox::MBOX;
use bcm2837::uart0::UART0;
use core::fmt::Write;
use display::Display;
use mailbox::{
    channel, msg::blank_screen::BlankScreenCmd, msg::get_temperature::GetTemperatureCmd, msg::Resp,
    Mailbox,
};
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

    let mut display = Display::new(None, &mut mbox).unwrap();

    loop {
        let cmd = BlankScreenCmd { state: true };

        // Clear the display
        mbox.call(channel::PROP, &cmd).ok();

        let cmd = GetTemperatureCmd { id: 0 };

        // Get temperature 1/1000 degree C
        let resp = mbox.call(channel::PROP, &cmd);

        let temp: Option<u32> = if let Ok(resp) = resp {
            if let Resp::GetTemperatureResp(resp) = resp {
                Some(resp.value)
            } else {
                None
            }
        } else {
            None
        };

        display.draw(
            Font12x16::render_str("Hello World!")
                .with_stroke(Some((0xFF, 0xFF, 0xFF).into()))
                .translate(Coord::new(5, 45))
                .into_iter(),
        );
    }
}
