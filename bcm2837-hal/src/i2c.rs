//! I2C

// TODO
// - macro gen I2C2/x
// - support speeds other than 100k
// - is BSC2 usable?
// - nb blocking support?

// some other examples
// https://github.com/mpod/raspi-bare-metal/blob/master/src/bcm2835.c#L713
// https://github.com/bootc/linux/blob/rpi-3.2.19/drivers/i2c/busses/i2c-bcm2708.c

use crate::clocks::Clocks;
use crate::gpio::{Alternate, Pin0, Pin1, Pin2, Pin3, AF0};
use crate::hal::blocking::i2c::{Read, Write, WriteRead};
use crate::time::{Hertz, KiloHertz};
use bcm2837::{bsc0::*, bsc1::I2C1};

/// I2C error
#[derive(Debug)]
pub enum Error {
    /// No acknowledge returned
    Nack,
    /// Slave held the SCL low for longer than specified
    ClockStretchTimeout,
    #[doc(hidden)]
    _Extensible,
}

pub trait Pins<I2c> {}
pub trait PinScl<I2c> {}
pub trait PinSda<I2c> {}

impl<I2c, SCL, SDA> Pins<I2c> for (SCL, SDA)
where
    SCL: PinScl<I2c>,
    SDA: PinSda<I2c>,
{
}

impl PinSda<I2C0> for Pin0<Alternate<AF0>> {}
impl PinScl<I2C0> for Pin1<Alternate<AF0>> {}

impl PinSda<I2C1> for Pin2<Alternate<AF0>> {}
impl PinScl<I2C1> for Pin3<Alternate<AF0>> {}

/// I2C abstraction
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
}

impl<PINS> I2c<I2C0, PINS> {
    pub fn i2c0(i2c: I2C0, pins: PINS, _speed: KiloHertz, clocks: Clocks) -> Self
    where
        PINS: Pins<I2C0>,
    {
        // Reset, clear status bits
        i2c.CTRL.set(0);
        i2c.STATUS
            .modify(STATUS::CLKT::SET + STATUS::ERR::SET + STATUS::DONE::SET);

        // TODO - only 100k supported
        let speed: Hertz = KiloHertz(100).into();
        let cdiv = clocks.apbclk().0 / speed.0;

        i2c.DIV.modify(DIV::CDIV.val(cdiv));

        i2c.CTRL.modify(CTRL::I2CEN::SET + CTRL::CLEAR::ClearFifo);

        I2c { i2c, pins }
    }

    pub fn free(self) -> (I2C0, PINS) {
        (self.i2c, self.pins)
    }

    #[inline]
    fn recv_byte(&self) -> Result<u8, Error> {
        while !self.i2c.STATUS.is_set(STATUS::RXD) {}
        Ok(self.i2c.FIFO.read(FIFO::DATA) as u8)
    }

    #[inline]
    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        while !self.i2c.STATUS.is_set(STATUS::TXD) {}
        self.i2c.FIFO.modify(FIFO::DATA.val(byte as _));
        Ok(())
    }
}

impl<PINS> Read for I2c<I2C0, PINS> {
    type Error = Error;

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // Clear FIFO
        self.i2c.CTRL.modify(CTRL::CLEAR::ClearFifo);

        // Clear status
        self.i2c
            .STATUS
            .modify(STATUS::CLKT::SET + STATUS::ERR::SET + STATUS::DONE::SET);

        // Set data length
        self.i2c.DLEN.modify(DLEN::DLEN.val(buffer.len() as _));

        // Set slave address
        self.i2c.SA.modify(SA::ADDR.val(addr as _));

        // Start read
        self.i2c.CTRL.modify(CTRL::ST::SET + CTRL::RW::ReadTransfer);

        for c in buffer {
            *c = self.recv_byte()?;
        }

        // TODO - check done?
        //while !self.i2c.STATUS.is_set(STATUS::DONE) {
        assert_eq!(self.i2c.STATUS.is_set(STATUS::DONE), true);

        let result = if self.i2c.STATUS.is_set(STATUS::ERR) {
            Err(Error::Nack)
        } else if self.i2c.STATUS.is_set(STATUS::CLKT) {
            Err(Error::ClockStretchTimeout)
        } else {
            Ok(())
        };

        // Clear done
        self.i2c.STATUS.modify(STATUS::DONE::SET);

        result
    }
}

impl<PINS> Write for I2c<I2C0, PINS> {
    type Error = Error;

    fn write(&mut self, addr: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        // Clear FIFO
        self.i2c.CTRL.modify(CTRL::CLEAR::ClearFifo);

        // Clear status
        self.i2c
            .STATUS
            .modify(STATUS::CLKT::SET + STATUS::ERR::SET + STATUS::DONE::SET);

        // Set data length
        self.i2c.DLEN.modify(DLEN::DLEN.val(buffer.len() as _));

        // Set slave address
        self.i2c.SA.modify(SA::ADDR.val(addr as _));

        // Start write
        self.i2c
            .CTRL
            .modify(CTRL::ST::SET + CTRL::RW::WriteTransfer);

        for c in buffer {
            self.send_byte(*c)?;
        }

        // TODO - check done?
        //while !self.i2c.STATUS.is_set(STATUS::DONE) {
        assert_eq!(self.i2c.STATUS.is_set(STATUS::DONE), true);

        let result = if self.i2c.STATUS.is_set(STATUS::ERR) {
            Err(Error::Nack)
        } else if self.i2c.STATUS.is_set(STATUS::CLKT) {
            Err(Error::ClockStretchTimeout)
        } else {
            Ok(())
        };

        // Clear done
        self.i2c.STATUS.modify(STATUS::DONE::SET);

        result
    }
}

impl<PINS> WriteRead for I2c<I2C0, PINS> {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.write(addr, bytes)?;
        self.read(addr, buffer)?;

        Ok(())
    }
}
