use bcm2837::uart0::UART0;
use core::fmt::Write;
use core::intrinsics;
use core::panic::PanicInfo;

use serial::Serial;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Hopefully UART was already configured
    let mut serial = Serial::new(UART0::new());
    writeln!(serial, "{}", info).ok();

    unsafe { intrinsics::abort() }
}
