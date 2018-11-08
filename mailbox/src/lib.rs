#![no_std]

extern crate bcm2837;
extern crate cortex_a;

use bcm2837::mbox::MBOX;
use bcm2837::mbox::STATUS;
use core::sync::atomic::{compiler_fence, Ordering};
use cortex_a::asm;

// TODO - cleanup organization of modules
pub mod msg;

pub trait MailboxBufferConstructor {
    fn construct_buffer(&self, buffer: &mut [u32; MAILBOX_BUFFER_LEN]);
}

// Cmd/Resp constructors?
// or single trait with construct_cmd/_resp()?
// maybe impl From for Resp structs?
// Currently each will impl From<&[u32; MAILBOX_BUFFER_LEN]> for T

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
// TODO - move into a mod/enum, use by the cmd/resp
pub mod tag {
    //pub const GETSERIAL: u32 = 0x10004;
    //pub const SETCLKRATE: u32 = 0x38002;
    pub const LAST: u32 = 0;
}

// Clocks
pub mod clock {
    pub const UART: u32 = 0x0000_0002;
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

    /// Make a mailbox call. Returns Err(MboxError) on failure, Ok(()) success
    pub fn call<T: MailboxBufferConstructor>(
        &mut self,
        channel: u32,
        constructor: &T,
    ) -> Result<msg::Resp> {
        constructor.construct_buffer(&mut self.buffer);

        // Insert a compiler fence that ensures that all stores to the
        // mbox buffer are finished before the GPU is signaled (which
        // is done by a store operation as well).
        compiler_fence(Ordering::Release);

        // wait until we can write to the mailbox
        loop {
            if !self.mbox.STATUS.is_set(STATUS::FULL) {
                break;
            }

            asm::nop();
        }

        let buf_ptr = self.buffer.as_ptr() as u32;

        // write the address of our message to the mailbox with channel identifier
        self.mbox.WRITE.set((buf_ptr & !0xF) | (channel & 0xF));

        // now wait for the response
        loop {
            // is there a response?
            loop {
                if !self.mbox.STATUS.is_set(STATUS::EMPTY) {
                    break;
                }

                asm::nop();
            }

            let resp: u32 = self.mbox.READ.get();

            // is it a response to our message?
            if ((resp & 0xF) == channel) && ((resp & !0xF) == buf_ptr) {
                // is it a valid successful response?
                return match self.buffer[1] {
                    response::SUCCESS => Ok(msg::Resp::from(&self.buffer)),
                    response::ERROR => Err(MboxError::ResponseError),
                    _ => Err(MboxError::UnknownError),
                };
            }
        }
    }
}
