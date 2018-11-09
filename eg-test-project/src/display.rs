use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::PixelColorU32;
use embedded_graphics::Drawing;
use mailbox::msg::framebuffer::FramebufferResp;

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

impl Drawing<PixelColorU32> for Display {
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = Pixel<PixelColorU32>>,
    {
        for Pixel(coord, color) in item_pixels {
            if coord[0] >= self.fb_data.phy_width || coord[1] >= self.fb_data.phy_height {
                continue;
            }

            self.fb_data
                .set_pixel(coord[0], coord[1], color.into_inner());
        }
    }
}
