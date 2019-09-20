#![no_std]
#![no_main]

extern crate bcm2837_hal as hal;

use crate::hal::bcm2837::gpio::GPIO;
use crate::hal::bcm2837::mbox::MBOX;
use crate::hal::bcm2837::sys_timer::SysTimer;
use crate::hal::bcm2837::uart1::UART1;
use crate::hal::clocks::Clocks;
use crate::hal::mailbox::Mailbox;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use core::fmt::Write;

fn kernel_entry() -> ! {
    let mut mbox = Mailbox::new(MBOX::new());
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    let gpio = GPIO::new();

    let gp = gpio.split();
    let tx = gp.p14.into_alternate_af0();
    let rx = gp.p15.into_alternate_af0();

    let mut serial = Serial::uart1(UART1::new(), (tx, rx), 0, clocks);

    let sys_timer = SysTimer::new();
    let mut sys_counter = sys_timer.split().sys_counter;

    writeln!(serial, "{:#?}", clocks).ok();

    loop {
        writeln!(serial, "Hello World").ok();
        sys_counter.delay_ms(500u32);
    }
}

raspi3_boot::entry!(kernel_entry);
