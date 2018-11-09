// TODO - replace this with HAL layer bits

// TODO - cleanup
use bcm2837::gpio;
use bcm2837::uart0::UART0;
use bcm2837::uart0::*;
use cortex_a::asm;
use mailbox::msg::set_clock_rate::SetClockRateCmd;
use mailbox::Mailbox;
use mailbox::{channel, clock};

pub enum SerialError {
    MailboxError,
}

pub type Result<T> = ::core::result::Result<T, SerialError>;

pub struct Serial {
    uart: UART0,
}

impl Serial {
    pub fn new(uart: UART0) -> Self {
        Self { uart }
    }

    pub fn init(&self, mbox: &mut Mailbox) -> Result<()> {
        // turn off UART0
        self.uart.CR.set(0);

        // set up clock for consistent divisor values
        let cmd = SetClockRateCmd {
            clock_id: clock::UART,
            freq: 4_000_000,
            flags: 0, // skip turbo setting
        };

        if mbox.call(channel::PROP, &cmd).is_err() {
            return Err(SerialError::MailboxError); // Abort if UART clocks couldn't be set
        };

        // map UART0 to GPIO pins
        unsafe {
            (*gpio::GPFSEL1).modify(gpio::GPFSEL1::FSEL14::TXD0 + gpio::GPFSEL1::FSEL15::RXD0);

            (*gpio::GPPUD).set(0); // enable pins 14 and 15
            for _ in 0..150 {
                asm::nop();
            }

            (*gpio::GPPUDCLK0).write(
                gpio::GPPUDCLK0::PUDCLK14::AssertClock + gpio::GPPUDCLK0::PUDCLK15::AssertClock,
            );
            for _ in 0..150 {
                asm::nop();
            }

            (*gpio::GPPUDCLK0).set(0);
        }

        self.uart.ICR.write(ICR::ALL::CLEAR);
        self.uart.IBRD.write(IBRD::IBRD.val(2)); // Results in 115200 baud
        self.uart.FBRD.write(FBRD::FBRD.val(0xB));
        self.uart.LCRH.write(LCRH::WLEN::EightBit); // 8N1
        self.uart
            .CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Ok(())
    }

    /// Send a character
    pub fn send(&self, c: char) {
        // wait until we can send
        loop {
            if !self.uart.FR.is_set(FR::TXFF) {
                break;
            }

            asm::nop();
        }

        // write the character to the buffer
        self.uart.DR.set(c as u32);
    }

    /// Receive a character
    pub fn getc(&self) -> char {
        // wait until something is in the buffer
        loop {
            if !self.uart.FR.is_set(FR::RXFE) {
                break;
            }

            asm::nop();
        }

        // read it and return
        let mut ret = self.uart.DR.get() as u8 as char;

        // convert carrige return to newline
        if ret == '\r' {
            ret = '\n'
        }

        ret
    }

    /// Display a string
    pub fn puts(&self, string: &str) {
        for c in string.chars() {
            // convert newline to carrige return + newline
            if c == '\n' {
                self.send('\r')
            }

            self.send(c);
        }
    }
}

impl ::core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for &b in s.as_bytes() {
            self.send(b as _);
        }
        Ok(())
    }
}
