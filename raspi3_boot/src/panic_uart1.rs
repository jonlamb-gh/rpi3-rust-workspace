use crate::hal::bcm2837::gpio::GPIO;
use crate::hal::bcm2837::mbox::MBOX;
use crate::hal::bcm2837::uart1::UART1;
use crate::hal::clocks::Clocks;
use crate::hal::mailbox::Mailbox;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use core::fmt::Write;
use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut mbox = Mailbox::new(MBOX::new());
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    let gpio = GPIO::new();

    let gp = gpio.split();
    let tx = gp.p14.into_alternate_af0();
    let rx = gp.p15.into_alternate_af0();

    let mut serial = Serial::uart1(UART1::new(), (tx, rx), 0, clocks);
    writeln!(serial, "\n\n{}\n\n", info).ok();
    unsafe { intrinsics::abort() }
}
