#![no_std]
#![no_main]

extern crate bcm2837_hal as hal;

use crate::hal::bcm2837::bsc1::I2C1;
use crate::hal::bcm2837::gpio::GPIO;
use crate::hal::bcm2837::mbox::MBOX;
use crate::hal::bcm2837::sys_timer::SysTimer;
use crate::hal::bcm2837::uart1::UART1;
use crate::hal::clocks::Clocks;
use crate::hal::i2c::I2c;
use crate::hal::mailbox::Mailbox;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use core::fmt::Write as CoreWrite;

fn kernel_entry() -> ! {
    let mut mbox = Mailbox::new(MBOX::new());

    let clocks = Clocks::freeze(&mut mbox).unwrap();

    let mut gpio = GPIO::new();

    let sys_timer = SysTimer::new();
    let mut sys_counter = sys_timer.split().sys_counter;

    let mut serial = Serial::uart1(UART1::new(), 0, &mut gpio);

    // Split the GPIO device up into component Pin abstractions
    let gp = gpio.split();

    let sda = gp.p2.into_alternate_af0();
    let scl = gp.p3.into_alternate_af0();

    let mut i2c = I2c::i2c1(I2C1::new(), (scl, sda), 100.khz(), clocks);

    writeln!(serial, "I2C example").ok();

    loop {
        writeln!(serial, "Sending data").ok();
        i2c.write(0x68, &[0xAB, 0xBC]).unwrap();
        sys_counter.delay_ms(100u32);
    }
}

raspi3_boot::entry!(kernel_entry);
