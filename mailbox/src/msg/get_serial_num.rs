use super::super::tag;
use super::super::MailboxBufferConstructor;
use super::super::MAILBOX_BUFFER_LEN;
use super::super::REQUEST;

pub const TAG: u32 = 0x0001_0004;

pub const CMD_LEN: u32 = 0;
pub const RESP_LEN: u32 = 8;

#[derive(Debug)]
pub struct GetSerialNumCmd;

#[derive(Debug)]
pub struct GetSerialNumResp {
    pub serial_number: u64,
}

impl MailboxBufferConstructor for GetSerialNumCmd {
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
impl From<&[u32; MAILBOX_BUFFER_LEN]> for GetSerialNumResp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> GetSerialNumResp {
        assert_eq!(buffer[2], TAG);
        //assert_eq!(buffer[3], RESP_LEN);
        GetSerialNumResp {
            serial_number: buffer[5] as u64 | (buffer[6] as u64) << 32,
        }
    }
}
