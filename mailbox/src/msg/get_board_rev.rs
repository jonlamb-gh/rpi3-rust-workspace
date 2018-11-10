use super::super::tag;
use super::super::MailboxBufferConstructor;
use super::super::MAILBOX_BUFFER_LEN;
use super::super::REQUEST;

pub const TAG: u32 = 0x0001_0002;

pub const CMD_LEN: u32 = 0;
pub const RESP_LEN: u32 = 4;

#[derive(Debug)]
pub struct GetBoardRevCmd;

#[derive(Debug)]
pub struct GetBoardRevResp {
    pub board_revision: u32,
}

impl MailboxBufferConstructor for GetBoardRevCmd {
    fn construct_buffer(&self, buffer: &mut [u32; MAILBOX_BUFFER_LEN]) {
        buffer[0] = 6 * 4;
        buffer[1] = REQUEST;
        buffer[2] = TAG;
        buffer[3] = CMD_LEN;
        buffer[4] = RESP_LEN;
        buffer[5] = tag::LAST;
    }
}

// TODO - sanity checks/result?
impl From<&[u32; MAILBOX_BUFFER_LEN]> for GetBoardRevResp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> GetBoardRevResp {
        assert_eq!(buffer[2], TAG);
        //assert_eq!(buffer[3], RESP_LEN);
        GetBoardRevResp {
            board_revision: buffer[5],
        }
    }
}
