use super::super::tag;
use super::super::MailboxBufferConstructor;
use super::super::MAILBOX_BUFFER_LEN;
use super::super::REQUEST;

// TODO - currently up a level in Mailbox, should they
// be pushed down to this level?
pub const TAG: u32 = 0x0004_0002;

pub const CMD_LEN: u32 = 4;
pub const RESP_LEN: u32 = 4;

#[derive(Debug)]
pub struct BlankScreenCmd {
    pub state: bool,
}

#[derive(Debug)]
pub struct BlankScreenResp {
    pub state: bool,
}

impl MailboxBufferConstructor for BlankScreenCmd {
    fn construct_buffer(&self, buffer: &mut [u32; MAILBOX_BUFFER_LEN]) {
        // set up clock for consistent divisor values
        buffer[0] = 7 * 4;
        buffer[1] = REQUEST;
        buffer[2] = TAG;
        buffer[3] = CMD_LEN;
        buffer[4] = RESP_LEN;
        buffer[5] = self.state as u32 & 0x01;
        buffer[6] = tag::LAST;
    }
}

// TODO - sanity checks/result?
impl From<&[u32; MAILBOX_BUFFER_LEN]> for BlankScreenResp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> BlankScreenResp {
        assert_eq!(buffer[2], TAG);
        assert_eq!(buffer[3], RESP_LEN);
        BlankScreenResp {
            state: if buffer[5] & 0x1 == 0 { false } else { true },
        }
    }
}
