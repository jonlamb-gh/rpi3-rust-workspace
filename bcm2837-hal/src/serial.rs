//! Serial
//!
//! There are two built-in UARTS, a PL011 (UART0)
//! and a mini UART (UART1).
//!
//! See the documentation:
//! https://www.raspberrypi.org/documentation/configuration/uart.md

use crate::clocks::Clocks;
use crate::gpio::{Alternate, Pin14, Pin15, AF0, AF5};
use crate::hal::prelude::*;
use crate::hal::serial;
use crate::time::Bps;
use bcm2837::uart0::*;
use bcm2837::uart1::*;
use core::fmt;
use nb::block;
use void::Void;

pub trait Pins<UART> {}
pub trait PinTx<UART> {}
pub trait PinRx<UART> {}

impl<UART, TX, RX> Pins<UART> for (TX, RX)
where
    TX: PinTx<UART>,
    RX: PinRx<UART>,
{
}

impl PinTx<UART0> for Pin14<Alternate<AF0>> {}
impl PinRx<UART0> for Pin15<Alternate<AF0>> {}

impl PinTx<UART1> for Pin14<Alternate<AF5>> {}
impl PinRx<UART1> for Pin15<Alternate<AF5>> {}

/// Serial abstraction
pub struct Serial<UART, PINS> {
    uart: UART,
    pins: PINS,
}

impl<PINS> Serial<UART0, PINS> {
    pub fn uart0(uart: UART0, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<UART0>,
    {
        let brr = if baud_rate.0 > (clocks.uart().0 / 16) {
            (clocks.uart().0 * 8) / baud_rate.0
        } else {
            (clocks.uart().0 * 4) / baud_rate.0
        };

        // Turn off UART0
        uart.CR.set(0);

        uart.ICR.write(ICR::ALL::CLEAR);
        uart.IBRD.write(IBRD::IBRD.val(brr >> 6));
        uart.FBRD.write(FBRD::FBRD.val(brr & 0x3F));
        uart.LCRH.write(LCRH::WLEN::EightBit); // 8N1
        uart.CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Serial { uart, pins }
    }

    pub fn free(self) -> (UART0, PINS) {
        (self.uart, self.pins)
    }
}

impl<PINS> serial::Write<u8> for Serial<UART0, PINS> {
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

impl<PINS> fmt::Write for Serial<UART0, PINS> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
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

impl<PINS> Serial<UART1, PINS> {
    pub fn uart1(uart: UART1, pins: PINS, baud_rate: Bps, clocks: Clocks) -> Self
    where
        PINS: Pins<UART1>,
    {
        // Mini UART uses 8-times oversampling
        // baudrate_reg = ((sys_clock / baudrate) / 8) - 1
        let brr = ((clocks.core().0 / baud_rate.0) / 8) - 1;

        uart.AUX_ENABLES.modify(AUX_ENABLES::MINI_UART_ENABLE::SET);
        uart.AUX_MU_IER.set(0);
        uart.AUX_MU_CNTL.set(0);
        uart.AUX_MU_LCR.write(AUX_MU_LCR::DATA_SIZE::EightBit);
        uart.AUX_MU_MCR.set(0);
        uart.AUX_MU_IER.set(0);
        uart.AUX_MU_IIR.write(AUX_MU_IIR::FIFO_CLEAR::All);
        uart.AUX_MU_BAUD.write(AUX_MU_BAUD::RATE.val(brr));

        uart.AUX_MU_CNTL
            .write(AUX_MU_CNTL::RX_EN::Enabled + AUX_MU_CNTL::TX_EN::Enabled);

        Serial { uart, pins }
    }

    pub fn free(self) -> (UART1, PINS) {
        (self.uart, self.pins)
    }
}

impl<PINS> serial::Read<u8> for Serial<UART1, PINS> {
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

impl<PINS> serial::Write<u8> for Serial<UART1, PINS> {
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

impl<PINS> core::fmt::Write for Serial<UART1, PINS> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
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
