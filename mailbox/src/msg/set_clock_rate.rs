use super::super::tag;
use super::super::MailboxBufferConstructor;
use super::super::MAILBOX_BUFFER_LEN;
use super::super::REQUEST;

pub const TAG: u32 = 0x38002;

pub const CMD_LEN: u32 = 12;
pub const RESP_LEN: u32 = 8;

pub struct SetClockRateCmd {
    pub clock_id: u32,
    pub freq: u32,
    pub flags: u32,
}

pub struct SetClockRateResp {
    pub clock_id: u32,
    pub freq: u32,
}

impl MailboxBufferConstructor for SetClockRateCmd {
    fn construct_buffer(&self, buffer: &mut [u32; MAILBOX_BUFFER_LEN]) {
        // set up clock for consistent divisor values
        buffer[0] = 9 * 4;
        buffer[1] = REQUEST;
        buffer[2] = tag::SETCLKRATE;
        buffer[3] = 12;
        buffer[4] = 8;
        //mbox.buffer[5] = mbox::clock::UART; // UART clock
        buffer[6] = 4_000_000; // 4Mhz
        buffer[7] = 0; // skip turbo setting
        buffer[8] = tag::LAST;
    }
}
