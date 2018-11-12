// TODO
// - use types around digit/sec/min/hour/etc

use core::f32;
use display::{Display, ObjectDrawing};
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Circle;
use gui::{CircleDigit, CircleDigitConfig};
use rgb::RGB8;

//const DEGREE_PER_TICK: u32 = 6;
const TICK_PER_SECOND: u32 = 1;

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub center: Coord,
    pub radius: u32,
    pub outline_stroke_width: u8,
    pub outline_color: RGB8,
}

pub struct Clock {
    config: Config,
    sec_cd: CircleDigit,
}

impl Clock {
    pub fn new(config: Config) -> Self {
        let mut clock = Self {
            config,
            sec_cd: CircleDigit::new(CircleDigitConfig {
                center: Coord::new(300, 200),
                radius: 20,
                fill: true,
                text_color: RGB8::new(0xFF, 0xFF, 0xFF),
                background_fill_color: RGB8::new(0x0F, 0xAF, 0xF0),
                stroke_color: RGB8::new(0xFF, 0xFF, 0xFF),
                stroke_width: 2,
            }),
        };

        // TODO
        clock.update_digits(60);

        clock
    }

    fn update_digits(&mut self, sec: u32) {
        self.update_second_digit(sec);
    }

    fn update_second_digit(&mut self, digit: u32) {
        assert!(digit <= 60);
        let radius = self.config.radius
            - self.sec_cd.config().radius
            - self.sec_cd.config().stroke_width as u32
            - 1;
        let center = self.radial_coord(radius, digit * TICK_PER_SECOND);
        self.sec_cd.set_center(center);
        self.sec_cd.set_value(digit);
    }

    fn draw_second_digit(&self, display: &mut Display) {
        self.sec_cd.draw_object(display);
    }

    fn draw_outline_circles(&self, display: &mut Display) {
        display.draw(
            Circle::new(self.config.center, self.config.radius)
                //.with_fill(Some((0xFF, 0xFF, 0x00).into()))
                .with_stroke(Some(self.config.outline_color.into()))
                .with_stroke_width(self.config.outline_stroke_width)
                .into_iter(),
        );
    }

    // relative to center, 0:59
    fn radial_coord(&self, radius: u32, tick_num: u32) -> Coord {
        let (x, y) = rad_tick_to_cart(tick_num);

        // map into our coordinate system
        let w_x = x * radius as f32;
        let w_y = -y * radius as f32;

        self.config.center + Coord::new(w_x as i32, w_y as i32)
    }
}

impl ObjectDrawing for Clock {
    fn draw_object(&self, display: &mut Display) {
        // draw back to front
        self.draw_outline_circles(display);
        self.draw_second_digit(display);
    }
}

// TODO - enable float to get sin/cos/etc
// 60 is provided for convenience, returns same as 0
fn rad_tick_to_cart(tick_num: u32) -> (f32, f32) {
    assert!(tick_num <= 60);
    match tick_num {
        0 => (0_f32, 1_f32),
        1 => (0.104528464_f32, 0.9945219_f32),
        2 => (0.2079117_f32, 0.9781476_f32),
        3 => (0.309017_f32, 0.95105654_f32),
        4 => (0.40673664_f32, 0.9135454_f32),
        5 => (0.5_f32, 0.8660254_f32),
        6 => (0.58778524_f32, 0.809017_f32),
        7 => (0.6691306_f32, 0.7431448_f32),
        8 => (0.74314487_f32, 0.66913056_f32),
        9 => (0.809017_f32, 0.5877853_f32),
        10 => (0.86602545_f32, 0.49999997_f32),
        11 => (0.9135455_f32, 0.4067366_f32),
        12 => (0.95105654_f32, 0.30901697_f32),
        13 => (0.9781476_f32, 0.20791166_f32),
        14 => (0.9945219_f32, 0.10452842_f32),
        15 => (1_f32, -0.00000004371139_f32),
        16 => (0.9945219_f32, -0.10452851_f32),
        17 => (0.9781476_f32, -0.20791163_f32),
        18 => (0.95105654_f32, -0.30901694_f32),
        19 => (0.9135455_f32, -0.40673658_f32),
        20 => (0.8660254_f32, -0.50000006_f32),
        21 => (0.809017_f32, -0.5877852_f32),
        22 => (0.7431448_f32, -0.6691307_f32),
        23 => (0.6691306_f32, -0.7431448_f32),
        24 => (0.5877852_f32, -0.80901706_f32),
        25 => (0.50000006_f32, -0.8660254_f32),
        26 => (0.40673658_f32, -0.9135455_f32),
        27 => (0.30901703_f32, -0.9510565_f32),
        28 => (0.20791161_f32, -0.9781476_f32),
        29 => (0.104528494_f32, -0.9945219_f32),
        30 => (-0.00000008742278_f32, -1_f32),
        31 => (-0.104528435_f32, -0.9945219_f32),
        32 => (-0.20791179_f32, -0.97814757_f32),
        33 => (-0.30901697_f32, -0.95105654_f32),
        34 => (-0.40673652_f32, -0.9135455_f32),
        35 => (-0.49999997_f32, -0.8660254_f32),
        36 => (-0.5877851_f32, -0.80901706_f32),
        37 => (-0.6691306_f32, -0.7431448_f32),
        38 => (-0.74314475_f32, -0.6691307_f32),
        39 => (-0.8090168_f32, -0.5877854_f32),
        40 => (-0.86602545_f32, -0.4999999_f32),
        41 => (-0.9135454_f32, -0.40673664_f32),
        42 => (-0.9510565_f32, -0.3090171_f32),
        43 => (-0.97814757_f32, -0.20791192_f32),
        44 => (-0.9945219_f32, -0.10452834_f32),
        45 => (-1_f32, 0.000000011924881_f32),
        46 => (-0.9945219_f32, 0.10452836_f32),
        47 => (-0.9781476_f32, 0.20791148_f32),
        48 => (-0.9510565_f32, 0.30901712_f32),
        49 => (-0.9135454_f32, 0.40673667_f32),
        50 => (-0.86602545_f32, 0.4999999_f32),
        51 => (-0.8090171_f32, 0.58778507_f32),
        52 => (-0.74314475_f32, 0.66913074_f32),
        53 => (-0.66913056_f32, 0.74314487_f32),
        54 => (-0.5877853_f32, 0.80901694_f32),
        55 => (-0.5000002_f32, 0.8660253_f32),
        56 => (-0.4067365_f32, 0.91354555_f32),
        57 => (-0.30901694_f32, 0.95105654_f32),
        58 => (-0.20791176_f32, 0.97814757_f32),
        59 => (-0.10452865_f32, 0.99452186_f32),
        60 => (0_f32, 1_f32),
        _ => panic!("Invalid tick_num"),
    }
}
