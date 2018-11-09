use super::super::tag;
use super::super::MailboxBufferConstructor;
use super::super::MAILBOX_BUFFER_LEN;
use super::super::REQUEST;

// TODO - currently up a level in Mailbox, should they
// be pushed down to this level?
pub const TAG: u32 = 0x38002;

pub const CMD_LEN: u32 = 12;
pub const RESP_LEN: u32 = 8;

pub struct SetClockRateCmd {
    // TODO - enum
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
        buffer[2] = TAG;
        buffer[3] = CMD_LEN;
        buffer[4] = RESP_LEN;
        buffer[5] = self.clock_id;
        buffer[6] = self.freq;
        // skip turbo setting
        buffer[7] = 0;
        buffer[8] = tag::LAST;
    }
}

// TODO - sanity checks/result?
impl From<&[u32; MAILBOX_BUFFER_LEN]> for SetClockRateResp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> SetClockRateResp {
        // some of these can be moved up a level or so
        assert_eq!(buffer[2], TAG);
        //assert_eq!(buffer[4], RESP_LEN);
        SetClockRateResp {
            clock_id: buffer[5],
            freq: buffer[6],
        }
    }
}
