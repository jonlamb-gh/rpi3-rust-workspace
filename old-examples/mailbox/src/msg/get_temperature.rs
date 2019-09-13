use super::super::tag;
use super::super::MailboxBufferConstructor;
use super::super::MAILBOX_BUFFER_LEN;
use super::super::REQUEST;

pub const TAG: u32 = 0x0003_0006;

pub const CMD_LEN: u32 = 4;
pub const RESP_LEN: u32 = 8;

#[derive(Debug)]
pub struct GetTemperatureCmd {
    pub id: u32,
}

#[derive(Debug)]
pub struct GetTemperatureResp {
    pub id: u32,
    pub value: u32,
}

impl MailboxBufferConstructor for GetTemperatureCmd {
    fn construct_buffer(&self, buffer: &mut [u32; MAILBOX_BUFFER_LEN]) {
        buffer[0] = 7 * 4;
        buffer[1] = REQUEST;
        buffer[2] = TAG;
        buffer[3] = CMD_LEN;
        buffer[4] = RESP_LEN;
        buffer[5] = self.id;
        buffer[6] = tag::LAST;
    }
}

// TODO - sanity checks/result?
impl From<&[u32; MAILBOX_BUFFER_LEN]> for GetTemperatureResp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> GetTemperatureResp {
        assert_eq!(buffer[2], TAG);
        //assert_eq!(buffer[3], RESP_LEN);
        GetTemperatureResp {
            id: buffer[5],
            value: buffer[6],
        }
    }
}
