#![no_std]
#![no_main]

extern crate bcm2837_hal as hal;

use crate::hal::bcm2837::gpio::GPIO;
use crate::hal::bcm2837::sys_timer::SysTimer;
use crate::hal::bcm2837::uart1::UART1;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use core::fmt::Write;

fn kernel_entry() -> ! {
    let mut gpio = GPIO::new();
    let mut serial = Serial::uart1(UART1::new(), 0, &mut gpio);

    let sys_timer = SysTimer::new();
    let mut sys_counter = sys_timer.split().sys_counter;

    loop {
        writeln!(serial, "Hello World").ok();
        sys_counter.delay_ms(500u32);
    }
}

raspi3_boot::entry!(kernel_entry);
