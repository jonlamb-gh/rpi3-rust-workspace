//! Serial
//! UART0 and UART1 are significantly different so they both
//! have hand implementations rather than using a macro

use crate::hal::prelude::*;
use crate::hal::serial;
use bcm2837::gpio::*;
use bcm2837::uart0::*;
use bcm2837::uart1::*;
use cortex_a::asm;
use nb::block;
use void::Void;

pub struct Serial<UART> {
    uart: UART,
}

// TODO - consume pins 14 and 15

impl Serial<UART0> {
    // TODO
    // - needs to configure the clock using a mbox message
    // - time bits, Bps, etc
    pub fn uart0(uart: UART0, _baud_rate: u32, gpio: &mut GPIO) -> Self {
        // Turn off UART0
        uart.CR.set(0);

        // TODO - mbox clock configs

        // Map UART0 to GPIO pins
        gpio.GPFSEL1
            .modify(GPFSEL1::FSEL14::AF0 + GPFSEL1::FSEL15::AF0);

        gpio.GPPUD.set(0);
        for _ in 0..150 {
            asm::nop();
        }

        gpio.GPPUDCLK0
            .write(GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock);
        for _ in 0..150 {
            asm::nop();
        }
        gpio.GPPUDCLK0.set(0);

        uart.ICR.write(ICR::ALL::CLEAR);
        uart.IBRD.write(IBRD::IBRD.val(2)); // Results in 115200 baud
        uart.FBRD.write(FBRD::FBRD.val(0xB));
        uart.LCRH.write(LCRH::WLEN::EightBit); // 8N1
        uart.CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Serial { uart }
    }

    pub fn free(self) -> UART0 {
        self.uart
    }
}

impl serial::Write<u8> for Serial<UART0> {
    type Error = Void;

    fn flush(&mut self) -> nb::Result<(), Void> {
        if !self.uart.FR.is_set(FR::TXFF) {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
        if !self.uart.FR.is_set(FR::TXFF) {
            self.uart.DR.set(byte as _);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl ::core::fmt::Write for Serial<UART0> {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for b in s.bytes() {
            // Convert '\n' to '\r\n'
            if b as char == '\n' {
                block!(self.write('\r' as _)).ok();
            }
            block!(self.write(b)).ok();
        }
        Ok(())
    }
}

impl Serial<UART1> {
    // TODO - time bits, Bps, etc
    pub fn uart1(uart: UART1, _baud_rate: u32, gpio: &mut GPIO) -> Self {
        uart.AUX_ENABLES.modify(AUX_ENABLES::MINI_UART_ENABLE::SET);
        uart.AUX_MU_IER.set(0);
        uart.AUX_MU_CNTL.set(0);
        uart.AUX_MU_LCR.write(AUX_MU_LCR::DATA_SIZE::EightBit);
        uart.AUX_MU_MCR.set(0);
        uart.AUX_MU_IER.set(0);
        uart.AUX_MU_IIR.write(AUX_MU_IIR::FIFO_CLEAR::All);
        uart.AUX_MU_BAUD.write(AUX_MU_BAUD::RATE.val(270)); // 115200 baud

        // Map UART1 to GPIO pins
        gpio.GPFSEL1
            .modify(GPFSEL1::FSEL14::AF5 + GPFSEL1::FSEL15::AF5);

        // Enable pins 14 and 15
        gpio.GPPUD.set(0);
        for _ in 0..150 {
            asm::nop();
        }
        gpio.GPPUDCLK0
            .write(GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock);
        for _ in 0..150 {
            asm::nop();
        }
        gpio.GPPUDCLK0.set(0);

        uart.AUX_MU_CNTL
            .write(AUX_MU_CNTL::RX_EN::Enabled + AUX_MU_CNTL::TX_EN::Enabled);

        Serial { uart }
    }

    pub fn free(self) -> UART1 {
        self.uart
    }
}

impl serial::Read<u8> for Serial<UART1> {
    type Error = Void;

    fn read(&mut self) -> nb::Result<u8, Void> {
        if self.uart.AUX_MU_LSR.is_set(AUX_MU_LSR::DATA_READY) {
            let mut data = self.uart.AUX_MU_IO.get() as u8;

            // convert carrige return to newline
            if data == '\r' as _ {
                data = '\n' as _;
            }

            Ok(data)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl serial::Write<u8> for Serial<UART1> {
    type Error = Void;

    fn flush(&mut self) -> nb::Result<(), Void> {
        if self.uart.AUX_MU_LSR.is_set(AUX_MU_LSR::TX_EMPTY) {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Void> {
        if self.uart.AUX_MU_LSR.is_set(AUX_MU_LSR::TX_EMPTY) {
            self.uart.AUX_MU_IO.set(byte as _);
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl ::core::fmt::Write for Serial<UART1> {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for b in s.bytes() {
            // Convert '\n' to '\r\n'
            if b as char == '\n' {
                block!(self.write('\r' as _)).ok();
            }
            block!(self.write(b)).ok();
        }
        Ok(())
    }
}
