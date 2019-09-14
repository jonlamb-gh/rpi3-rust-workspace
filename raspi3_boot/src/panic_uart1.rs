use crate::hal::bcm2837::gpio::GPIO;
use crate::hal::bcm2837::uart1::UART1;
use crate::hal::serial::Serial;
use core::fmt::Write;
use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut gpio = GPIO::new();
    let mut serial = Serial::uart1(UART1::new(), 0, &mut gpio);
    writeln!(serial, "\n\n{}\n\n", info).ok();
    unsafe { intrinsics::abort() }
}
