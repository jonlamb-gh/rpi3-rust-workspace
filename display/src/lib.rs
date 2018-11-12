#![no_std]

extern crate embedded_graphics;
extern crate mailbox;
extern crate rgb;

use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::Drawing;
use mailbox::msg::blank_screen::BlankScreenCmd;
use mailbox::msg::framebuffer::{FramebufferCmd, FramebufferResp};
use mailbox::msg::Resp;
use mailbox::{channel, Mailbox, MboxError, Result};
use rgb::*;

// TODO - until I figure out how to cleanly use embedded-graphics IntoIterator
// to combine primitives,
// this can be used to pass around a mut Display
pub trait ObjectDrawing {
    fn draw_object(&self, display: &mut Display);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DisplayColor(pub RGB8);

impl PixelColor for DisplayColor {}

impl From<u8> for DisplayColor {
    #[inline]
    fn from(other: u8) -> Self {
        DisplayColor(RGB8::new(other, other, other))
    }
}

impl From<u16> for DisplayColor {
    #[inline]
    fn from(other: u16) -> Self {
        let mono = (other >> 1 & 0xFF) as u8;
        DisplayColor(RGB8::new(mono, mono, mono))
    }
}

impl From<u32> for DisplayColor {
    #[inline]
    fn from(other: u32) -> Self {
        DisplayColor(RGB8::new(
            (other & 0xFF) as u8,
            (other >> 8 & 0xFF) as u8,
            (other >> 16 & 0xFF) as u8,
        ))
    }
}

impl From<(u8, u8, u8)> for DisplayColor {
    #[inline]
    fn from(other: (u8, u8, u8)) -> Self {
        DisplayColor(RGB8::new(other.0, other.1, other.2))
    }
}

impl From<RGB8> for DisplayColor {
    #[inline]
    fn from(other: RGB8) -> Self {
        DisplayColor(other)
    }
}

impl From<DisplayColor> for u32 {
    #[inline]
    fn from(color: DisplayColor) -> u32 {
        0xFF_00_00_00 | color.0.r as u32 | (color.0.g as u32) << 8 | (color.0.b as u32) << 16
    }
}

impl DisplayColor {
    pub fn into_inner(self) -> RGB8 {
        self.0
    }
}

const DEFAULT_FB_CFG: FramebufferCmd = FramebufferCmd {
    phy_width: 240,
    phy_height: 240,

    virt_width: 240,
    virt_height: 240,

    x_offset: 0,
    y_offset: 0,
};

pub struct Display {
    fb_data: FramebufferResp,
}

impl Display {
    pub fn new(cfgcmd: Option<FramebufferCmd>, mbox: &mut Mailbox) -> Result<Self> {
        let cmd = if let Some(cfgcmd) = cfgcmd {
            cfgcmd
        } else {
            DEFAULT_FB_CFG
        };

        let resp = mbox.call(channel::PROP, &cmd)?;

        if let Resp::FramebufferResp(fb_resp) = resp {
            Ok(Display::from(fb_resp))
        } else {
            Err(MboxError::ResponseError)
        }
    }

    pub fn width(&self) -> u32 {
        self.fb_data.phy_width
    }

    pub fn height(&self) -> u32 {
        self.fb_data.phy_height
    }

    #[inline]
    pub fn clear_screen(&self, mbox: &mut Mailbox) {
        let cmd = BlankScreenCmd { state: true };
        mbox.call(channel::PROP, &cmd).ok();
    }
}

impl From<FramebufferResp> for Display {
    fn from(resp: FramebufferResp) -> Display {
        Display { fb_data: resp }
    }
}

impl Drawing<DisplayColor> for Display {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = Pixel<DisplayColor>>,
    {
        for Pixel(coord, color) in item_pixels {
            if coord[0] >= self.fb_data.phy_width || coord[1] >= self.fb_data.phy_height {
                continue;
            }

            self.fb_data.set_pixel(coord[0], coord[1], u32::from(color));
        }
    }
}
