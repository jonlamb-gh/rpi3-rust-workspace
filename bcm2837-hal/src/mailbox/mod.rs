use crate::pmem::PMem;
use bcm2837::mbox::*;
use core::convert::TryFrom;
use core::sync::atomic::{compiler_fence, Ordering};
use cortex_a::{asm, barrier};

mod msg;
mod tag_id;

pub use crate::mailbox::msg::*;
pub use crate::mailbox::tag_id::TagId;

pub type Result<T> = core::result::Result<T, Error>;

// TODO - redo these
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    UnkownTagId(u32),
    Truncated,
    MessageWordAlign,
    Malformed,
    /// The response buffer has error bit(s) set
    BadRequest,
    /// Status word was not recognized
    BadStatusWord,
    /// Unknown error
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Channel {
    /// Property channel
    Prop = 8,
}

impl From<Channel> for u32 {
    fn from(c: Channel) -> u32 {
        c as u32
    }
}

/// Mailbox abstraction
pub struct Mailbox {
    mbox: MBOX,
    buffer_pmem: PMem,
}

impl Mailbox {
    pub fn new(mbox: MBOX, buffer_pmem: PMem) -> Result<Self> {
        if buffer_pmem.size() < BUFFER_SIZE {
            Err(Error::Truncated)
        } else {
            Ok(Mailbox { mbox, buffer_pmem })
        }
    }

    /// Returns a newly allocated high-level representation of the response
    pub fn call<R: MsgEmitter>(&mut self, channel: Channel, req: &R) -> Result<RespMsg> {
        // TODO - add size/etc utils for new_checked() fn's
        //
        // TODO - check resp capacity

        let mut msg =
            unsafe { Msg::new_unchecked(self.buffer_pmem.as_mut_slice::<u32>(BUFFER_LEN)) };

        // Emit into our local buffer
        req.emit_msg(&mut msg)?;

        // Insert a compiler fence that ensures that all stores to the
        // mbox buffer are finished before the GPU is signaled (which
        // is done by a store operation as well).
        compiler_fence(Ordering::Release);

        // Wait until we can write to the mailbox
        loop {
            if self.mbox.STATUS.is_set(STATUS::FULL) == false {
                break;
            }
            asm::nop();
        }

        // Write the physical address of our message
        // to the mailbox with channel identifier
        let buffer_paddr = self.buffer_pmem.paddr();
        self.mbox
            .WRITE
            .set((buffer_paddr & !0xF) | (u32::from(channel) & 0xF));

        // Wait for a response
        loop {
            loop {
                if self.mbox.STATUS.is_set(STATUS::EMPTY) == false {
                    break;
                }
                asm::nop();
            }

            let resp_word = self.mbox.READ.get();

            // Check if it is a response to our message
            if ((resp_word & 0xF) == channel.into()) && ((resp_word & !0xF) == buffer_paddr) {
                unsafe { barrier::dmb(barrier::SY) };

                let msg =
                    unsafe { Msg::new_checked(self.buffer_pmem.as_slice::<u32>(BUFFER_SIZE)) }?;

                return match msg.reqresp_code() {
                    ReqRespCode::ResponseSuccess => Ok(RespMsg::try_from(msg)?),
                    ReqRespCode::ResponseError => Err(Error::BadRequest),
                    _ => Err(Error::BadStatusWord),
                };
            }
        }
    }
}
