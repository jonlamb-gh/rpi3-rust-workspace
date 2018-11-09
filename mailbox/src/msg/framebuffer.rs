//! Framebuffer commands and responses
//! TODO - break this up, iterate over cmd/resp messages, combine for single op

use core::ptr;

use super::super::tag;
use super::super::MailboxBufferConstructor;
use super::super::MAILBOX_BUFFER_LEN;
use super::super::REQUEST;

pub const ALLOC_BUFFER_TAG: u32 = 0x40001;
pub const GET_PITCH_TAG: u32 = 0x40008;
pub const SET_PHY_SIZE_TAG: u32 = 0x48003;
pub const SET_VIRT_SIZE_TAG: u32 = 0x48004;
pub const SET_DEPTH_TAG: u32 = 0x48005;
pub const SET_PIXEL_ORDER_TAG: u32 = 0x48006;
pub const SET_VIRT_OFFSET_TAG: u32 = 0x48009;

#[derive(Debug)]
pub struct FramebufferCmd {
    pub phy_width: u32,
    pub phy_height: u32,

    pub virt_width: u32,
    pub virt_height: u32,

    pub x_offset: u32,
    pub y_offset: u32,
}

#[derive(Debug)]
pub struct FramebufferResp {
    // TODO - what else is useful?
    pub phy_width: u32,
    pub phy_height: u32,

    pub pitch: u32,
    pixels_ptr: *mut u32,
}

// TODO - here or where?
// impl FramebufferResp
// fn set_pixel(...)

impl MailboxBufferConstructor for FramebufferCmd {
    fn construct_buffer(&self, buffer: &mut [u32; MAILBOX_BUFFER_LEN]) {
        buffer[0] = 35 * 4;
        buffer[1] = REQUEST;

        buffer[2] = SET_PHY_SIZE_TAG;
        buffer[3] = 8;
        buffer[4] = 8;
        buffer[5] = self.phy_width;
        buffer[6] = self.phy_height;

        buffer[7] = SET_VIRT_SIZE_TAG;
        buffer[8] = 8;
        buffer[9] = 8;
        buffer[10] = self.virt_width;
        buffer[11] = self.virt_height;

        buffer[12] = SET_VIRT_OFFSET_TAG;
        buffer[13] = 8;
        buffer[14] = 8;
        buffer[15] = self.x_offset;
        buffer[16] = self.y_offset;

        buffer[17] = SET_DEPTH_TAG;
        buffer[18] = 4;
        buffer[19] = 4;
        buffer[20] = 32;

        buffer[21] = SET_PIXEL_ORDER_TAG;
        buffer[22] = 4;
        buffer[23] = 4;
        // RGB
        buffer[24] = 1;

        buffer[25] = ALLOC_BUFFER_TAG;
        buffer[26] = 8;
        buffer[27] = 8;
        buffer[28] = 4096;
        buffer[29] = 0;

        buffer[30] = GET_PITCH_TAG;
        buffer[31] = 4;
        buffer[32] = 4;
        buffer[33] = 0;

        buffer[34] = tag::LAST;
    }
}

// TODO - sanity checks/result?
impl From<&[u32; MAILBOX_BUFFER_LEN]> for FramebufferResp {
    fn from(buffer: &[u32; MAILBOX_BUFFER_LEN]) -> FramebufferResp {
        // depth
        assert_eq!(buffer[20], 32);
        // buffer
        assert_ne!(buffer[28], 0);

        FramebufferResp {
            phy_width: buffer[5],
            phy_height: buffer[6],
            pitch: buffer[33],
            pixels_ptr: (buffer[28] & 0x3FFF_FFFF) as *mut _,
        }
    }
}

impl FramebufferResp {
    /// RGB b[0] = Red, b[1] = Green, b[2] = Blue, b[3] = NA
    pub fn set_pixel(&mut self, x: u32, y: u32, value: u32) {
        let offset = (y * (self.pitch / 4)) + x;
        unsafe { ptr::write(self.pixels_ptr.offset(offset as _), value) };
    }
}
