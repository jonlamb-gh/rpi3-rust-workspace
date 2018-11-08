use super::MAILBOX_BUFFER_LEN;

pub mod framebuffer;
pub mod set_clock_rate;

use self::framebuffer::FramebufferResp;
use self::set_clock_rate::SetClockRateResp;

pub enum Resp {
    Ack,
    SetClockRateResp(SetClockRateResp),
    FramebufferResp(FramebufferResp),
}

// TODO - sanity checks/result?
impl From<&[u32; MAILBOX_BUFFER_LEN]> for Resp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> Resp {
        match buffer[2] {
            set_clock_rate::TAG => Resp::SetClockRateResp(SetClockRateResp::from(buffer)),
            // gate on the first tag
            framebuffer::SET_PHY_SIZE_TAG => Resp::FramebufferResp(FramebufferResp::from(buffer)),
            _ => Resp::Ack,
        }
    }
}
