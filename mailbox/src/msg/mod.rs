use super::MAILBOX_BUFFER_LEN;

pub mod blank_screen;
pub mod framebuffer;
pub mod get_temperature;
pub mod set_clock_rate;

use self::blank_screen::BlankScreenResp;
use self::framebuffer::FramebufferResp;
use self::get_temperature::GetTemperatureResp;
use self::set_clock_rate::SetClockRateResp;

#[derive(Debug)]
pub enum Resp {
    Ack,
    SetClockRateResp(SetClockRateResp),
    BlankScreenResp(BlankScreenResp),
    FramebufferResp(FramebufferResp),
    GetTemperatureResp(GetTemperatureResp),
}

// TODO - sanity checks/result?
impl From<&[u32; MAILBOX_BUFFER_LEN]> for Resp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> Resp {
        match buffer[2] {
            set_clock_rate::TAG => Resp::SetClockRateResp(SetClockRateResp::from(buffer)),
            blank_screen::TAG => Resp::BlankScreenResp(BlankScreenResp::from(buffer)),
            get_temperature::TAG => Resp::GetTemperatureResp(GetTemperatureResp::from(buffer)),
            // gate on the first tag
            framebuffer::SET_PHY_SIZE_TAG => Resp::FramebufferResp(FramebufferResp::from(buffer)),
            _ => Resp::Ack,
        }
    }
}
