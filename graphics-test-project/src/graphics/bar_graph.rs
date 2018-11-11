// TODO
// - error/sanity checks
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
use rgb::RGB8;
//use display::DisplayColor;
use display::Display;
use heapless::consts::U32;
use heapless::String;

// TODO - use Style<RGB8>?
pub struct Config {
    pub top_left: Coord,
    pub bottom_right: Coord,
    pub background_color: RGB8,
    pub fill_color: RGB8,
    pub text_color: RGB8,
    pub stroke_color: RGB8
    //style: Style,
}

pub struct BarGraph {
    config: Config,
    value: f32,
    width: i32,
    height: i32,
    center_x: i32,
    center_y: i32,
}

impl BarGraph {
    pub fn new(config: Config) -> Self {
        // precompute some commonly used bits
        let width: i32 = config.bottom_right.abs().0 - config.top_left.abs().0;
        let height: i32 = config.bottom_right.abs().1 - config.top_left.abs().1;
        let center_x: i32 = config.top_left.0 + (width / 2);
        let center_y: i32 = config.top_left.1 + (height / 2);

        Self {
            config,
            value: 0.2,
            width,
            height,
            center_x,
            center_y,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = if value <= 0.0 {
            0.0
        } else if value >= 1.0 {
            1.0
        } else {
            value
        };
    }

    pub fn test_draw(&self, display: &mut Display) {
        let mut value_str: String<U32> = String::new();
        write!(value_str, "{:.*}", 0, 100.0 * self.value).ok();

        let scaled = self.value * (self.height as f32);
        let fill_dist = scaled as i32;

        // drawing back to front, start with the fill
        if fill_dist > 0 {
            display.draw(
                Rect::new(
                    Coord::new(self.config.top_left.0, self.config.bottom_right.1 - fill_dist),
                    self.config.bottom_right,
                ).with_fill(Some((0x00, 0xAF, 0xCF).into()))
                .into_iter(),
            );
        }

        // TODO - fill color only set when fill/value exceeds text position?
        // or just set a background color and float the value above it until filled?
        let text = Font12x16::render_str(&value_str)
            .with_fill(Some((0x00, 0xAF, 0xCF).into()))
            .with_stroke(Some((0xFF, 0xFF, 0xFF).into()));
        display.draw(
            text.translate(Coord::new(
                self.center_x - (text.dimensions().0 as i32 / 2),
                self.center_y,
            )).into_iter(),
        );

        display.draw(
            Rect::new(self.config.top_left, self.config.bottom_right)
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
