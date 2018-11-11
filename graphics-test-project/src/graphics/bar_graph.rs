// TODO
// - horizontal/vertical
// - value/label
// - config/style/fonts/colors/etc

use core::fmt::Write;
use embedded_graphics::coord::Coord;
use embedded_graphics::fonts::Font12x16;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rect;
//use embedded_graphics::style::Style;
//use embedded_graphics::drawable::Pixel;
//use embedded_graphics::pixelcolor::PixelColor;
//use display::DisplayColor;
use display::Display;
use heapless::consts::U32;
use heapless::String;

pub struct BarGraph {
    top_left: Coord,
    bottom_right: Coord,
    //style: Style,
}

impl BarGraph {
    pub fn new(top_left: Coord, bottom_right: Coord) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    /*
    pub fn set_value(&mut self, value) {

    }
    */

    pub fn test_draw(&self, display: &mut Display, value: f32) {
        let mut value_str: String<U32> = String::new();
        write!(value_str, "{:.*}", 0, 100.0 * value).ok();

        let clamped = if value <= 0.0 {
            0.0
        } else if value >= 1.0 {
            1.0
        } else {
            value
        };

        let scaled = clamped * (self.bottom_right.abs().1 - self.top_left.abs().1) as f32;
        let fill_dist = scaled as i32;

        // draw back to front

        if fill_dist > 0 {
            display.draw(
                Rect::new(
                    Coord::new(self.top_left.0, self.bottom_right.1 - fill_dist),
                    self.bottom_right,
                ).with_fill(Some((0x00, 0xAF, 0xCF).into()))
                .into_iter(),
            );
        }

        // TODO - fill color only set when fill/value exceeds text position?
        // or just set a background color and float the value above it until filled?
        let text = Font12x16::render_str(&value_str)
            .with_fill(Some((0x00, 0xAF, 0xCF).into()))
            .with_stroke(Some((0xFF, 0xFF, 0xFF).into()));
        let center_x: i32 =
            self.top_left.0 + (self.bottom_right.abs().0 - self.top_left.abs().0) / 2;
        let center_y: i32 =
            self.top_left.1 + (self.bottom_right.abs().1 - self.top_left.abs().1) / 2;
        display.draw(
            text.translate(Coord::new(
                center_x - (text.dimensions().0 as i32 / 2),
                center_y,
            )).into_iter(),
        );

        display.draw(
            Rect::new(self.top_left, self.bottom_right)
                .with_stroke(Some((0xFF, 0xFF, 0xFF).into()))
                .with_stroke_width(2)
                .into_iter(),
        );
    }
}

/*
impl IntoIterator for BarGraph {
    type Item = Pixel<DisplayColor>;

    fn into_iter(self) -> Self::IntoIter {
    }
}
*/
