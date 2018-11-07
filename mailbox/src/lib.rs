#![no_std]

extern crate bcm2837;
extern crate cortex_a;

use bcm2837::mbox::MBOX;

// Custom errors
pub enum MboxError {
    ResponseError,
    UnknownError,
}
pub type Result<T> = ::core::result::Result<T, MboxError>;

// Channels
pub mod channel {
    pub const PROP: u32 = 8;
}

// Tags
pub mod tag {
    pub const GETSERIAL: u32 = 0x10004;
    pub const SETCLKRATE: u32 = 0x38002;
    pub const LAST: u32 = 0;
}

// Clocks
pub mod clock {
    pub const UART: u32 = 0x0_0000_0002;
}

// Responses
mod response {
    pub const SUCCESS: u32 = 0x8000_0000;
    pub const ERROR: u32 = 0x8000_0001; // error parsing request buffer (partial response)
}

pub const REQUEST: u32 = 0;

pub const MAILBOX_BUFFER_LEN: usize = 36;

pub struct Mailbox {
    mbox: MBOX,
    buffer: [u32; MAILBOX_BUFFER_LEN],
}

impl Mailbox {
    pub fn new(mbox: MBOX) -> Self {
        Self {
            mbox,
            buffer: [0; MAILBOX_BUFFER_LEN],
        }
    }

    pub fn buffer(&mut self) -> &mut [u32; MAILBOX_BUFFER_LEN] {
        &mut self.buffer
    }
}
