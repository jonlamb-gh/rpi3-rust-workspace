use embedded_graphics::primitives::Rect;
use embedded_graphics::primitives::Circle;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
//use embedded_graphics::style::Style;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::pixelcolor::PixelColor;

//use display::DisplayColor;
use display::Display;

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

    pub fn test_draw(&self, display: &mut Display) {
        display.draw(
            Rect::new(self.top_left, self.bottom_right)
                .with_stroke(Some((0xFF, 0x00, 0x00).into()))
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
