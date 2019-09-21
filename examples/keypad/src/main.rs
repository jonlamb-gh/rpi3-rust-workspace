#![no_std]
#![no_main]

extern crate bcm2837_hal as hal;

use crate::hal::bcm2837::gpio::GPIO;
use crate::hal::bcm2837::mbox::MBOX;
use crate::hal::bcm2837::uart1::UART1;
use crate::hal::clocks::Clocks;
use crate::hal::gpio::*;
use crate::hal::mailbox::Mailbox;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use crate::hal::time::Bps;
use core::fmt::Write;
use keypad::{keypad_new, keypad_struct};

keypad_struct! {
    struct PhoneKeypad {
        rows: (
          Pin5<Input<PullUp>>,
          Pin6<Input<PullUp>>,
          Pin13<Input<PullUp>>,
          Pin19<Input<PullUp>>,
        ),
        columns: (
            Pin17<Output<PushPull>>,
            Pin27<Output<PushPull>>,
            Pin22<Output<PushPull>>,
        ),
    }
}

const CHAR_MAP: [[char; 3]; 4] = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#'],
];

fn kernel_entry() -> ! {
    let mut mbox = Mailbox::new(MBOX::new());
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    let gpio = GPIO::new();
    let gp = gpio.split();

    let tx = gp.p14.into_alternate_af5();
    let rx = gp.p15.into_alternate_af5();

    let mut serial = Serial::uart1(UART1::new(), (tx, rx), Bps(115200), clocks);

    let kp_r0 = gp.p5.into_pull_up_input();
    let kp_r1 = gp.p6.into_pull_up_input();
    let kp_r2 = gp.p13.into_pull_up_input();
    let kp_r3 = gp.p19.into_pull_up_input();

    let kp_c0 = gp.p17.into_push_pull_output();
    let kp_c1 = gp.p27.into_push_pull_output();
    let kp_c2 = gp.p22.into_push_pull_output();

    let keypad = keypad_new!(PhoneKeypad {
        rows: (kp_r0, kp_r1, kp_r2, kp_r3,),
        columns: (kp_c0, kp_c1, kp_c2,),
    });

    let keys = keypad.decompose();

    writeln!(serial, "Keypad example").ok();

    loop {
        for (row_index, row) in keys.iter().enumerate() {
            for (col_index, key) in row.iter().enumerate() {
                if key.is_low().unwrap() {
                    let c = CHAR_MAP[row_index][col_index];
                    writeln!(serial, "key '{}' at {}, {}", c, row_index, col_index).ok();
                }
            }
        }
    }
}

raspi3_boot::entry!(kernel_entry);
