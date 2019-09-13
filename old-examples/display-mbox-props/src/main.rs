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

mod panic_handler;
mod serial;

use bcm2837::mbox::MBOX;
use bcm2837::uart0::UART0;
use core::fmt::Write;
use display::Display;
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font12x16;
use embedded_graphics::prelude::*;
use heapless::consts::U32;
use heapless::String;
use mailbox::channel;
use mailbox::msg::get_board_model::GetBoardModelCmd;
use mailbox::msg::get_board_rev::GetBoardRevCmd;
use mailbox::msg::get_serial_num::GetSerialNumCmd;
use mailbox::msg::get_temperature::GetTemperatureCmd;
use mailbox::msg::Resp;
use mailbox::Mailbox;

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

    let mut cnt: u32 = 0;
    let mut value_str: String<U32> = String::from("NA");
    loop {
        display.clear_screen(&mut mbox);

        let temp = get_temp(&mut mbox);
        let board_model = get_board_model(&mut mbox);
        let board_rev = get_board_rev(&mut mbox);
        let serial_num = get_serial_num(&mut mbox);

        value_str.clear();
        write!(value_str, "{} C", temp).ok();
        display.draw(
            Font12x16::render_str(&value_str)
                .with_stroke(Some((0xFF, 0xFF, 0xFF).into()))
                .translate(Coord::new(5, 20))
                .into_iter(),
        );

        value_str.clear();
        write!(value_str, "board_model 0x:{:X}", board_model).ok();
        display.draw(
            Font12x16::render_str(&value_str)
                .with_stroke(Some((0xFF, 0x00, 0x00).into()))
                .translate(Coord::new(5, 45))
                .into_iter(),
        );

        value_str.clear();
        write!(value_str, "board_rev 0x:{:X}", board_rev).ok();
        display.draw(
            Font12x16::render_str(&value_str)
                .with_stroke(Some((0x00, 0xFF, 0x00).into()))
                .translate(Coord::new(5, 70))
                .into_iter(),
        );

        value_str.clear();
        write!(value_str, "serial_num 0x:{:X}", serial_num).ok();
        display.draw(
            Font12x16::render_str(&value_str)
                .with_stroke(Some((0x00, 0x00, 0xFF).into()))
                .translate(Coord::new(5, 95))
                .into_iter(),
        );

        cnt += 1;
        value_str.clear();
        write!(value_str, "cnt: {}", cnt).ok();
        display.draw(
            Font12x16::render_str(&value_str)
                .with_stroke(Some((0xFF, 0x00, 0xFF).into()))
                .translate(Coord::new(5, 120))
                .into_iter(),
        );
    }
}

// Get temperature in 1/1000 degree C
fn get_temp(mbox: &mut Mailbox) -> u32 {
    let cmd = GetTemperatureCmd { id: 0 };
    let resp = mbox
        .call(channel::PROP, &cmd)
        .expect("GetTemperatureCmd failed");

    if let Resp::GetTemperatureResp(resp) = resp {
        resp.value
    } else {
        panic!("Resp::GetTemperatureResp() failed");
    }
}

fn get_board_model(mbox: &mut Mailbox) -> u32 {
    let cmd = GetBoardModelCmd {};
    let resp = mbox
        .call(channel::PROP, &cmd)
        .expect("GetBoardModelCmd failed");

    if let Resp::GetBoardModelResp(resp) = resp {
        resp.board_model
    } else {
        panic!("Resp::GetBoardModelResp() failed");
    }
}

fn get_board_rev(mbox: &mut Mailbox) -> u32 {
    let cmd = GetBoardRevCmd {};
    let resp = mbox
        .call(channel::PROP, &cmd)
        .expect("GetBoardRevCmd failed");

    if let Resp::GetBoardRevResp(resp) = resp {
        resp.board_revision
    } else {
        panic!("Resp::GetBoardRevResp() failed");
    }
}

fn get_serial_num(mbox: &mut Mailbox) -> u64 {
    let cmd = GetSerialNumCmd {};
    let resp = mbox
        .call(channel::PROP, &cmd)
        .expect("GetSerialNumCmd failed");

    if let Resp::GetSerialNumResp(resp) = resp {
        resp.serial_number
    } else {
        panic!("Resp::GetSerialNumResp() failed");
    }
}
