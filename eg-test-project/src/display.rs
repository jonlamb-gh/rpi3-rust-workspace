use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::Drawing;
use mailbox::msg::framebuffer::FramebufferResp;
use rgb::*;

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

pub struct Display {
    fb_data: FramebufferResp,
}

impl Display {
    pub fn new(fb_data: FramebufferResp) -> Self {
        Self { fb_data }
    }

    pub fn width(&self) -> u32 {
        self.fb_data.phy_width
    }

    pub fn height(&self) -> u32 {
        self.fb_data.phy_height
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
