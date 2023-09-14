//! This library has definitions for various color types and assorted utilities for manipulating and
//! working with the RGBA8 color type. Colors are intended to be compatible with the
//! [smart-leds](https://github.com/smart-leds-rs/smart-leds) crate, which in turn is compatible
//! with the [rgb](https://github.com/kornelski/rust-rgb) crate.

use crate::utility::Progression;
use rgb::RGBA8;

pub fn color_lerp(
    factor: i32,
    in_min: i32,
    in_max: i32,
    start_color: RGBA8,
    end_color: RGBA8,
) -> RGBA8 {
    let lerp = |start: u8, end: u8| {
        let start = start as i32;
        let end = end as i32;
        ((factor - in_min) * (end - start) / (in_max - in_min) + start) as u8
    };
    let mut mid_color = <RGBA8>::new(0, 0, 0, 255);
    mid_color.r = lerp(start_color.r, end_color.r);
    mid_color.g = lerp(start_color.g, end_color.g);
    mid_color.b = lerp(start_color.b, end_color.b);
    mid_color.a = lerp(start_color.a, end_color.a);
    mid_color
}

pub trait ManipulatableColor<RgbType> {
    fn lerp_with(&self, to_color: RgbType, factor: Progression) -> RgbType;
    fn set_color(&mut self, c: RgbType);
}

impl ManipulatableColor<RGBA8> for RGBA8 {
    fn lerp_with(&self, to_color: RGBA8, factor: Progression) -> RGBA8 {
        color_lerp(
            factor.get_current() as i32,
            0,
            factor.total as i32,
            *self,
            to_color,
        )
    }

    fn set_color(&mut self, c: RGBA8) {
        self.r = c.r;
        self.g = c.g;
        self.b = c.b;
    }
}

// Generic colors:
pub const BLACK_A: RGBA8 = RGBA8 {
    r: 0x00,
    g: 0x00,
    b: 0x00,
    a: 0xFF,
};
pub const WHITE_A: RGBA8 = RGBA8 {
    r: 0xFF,
    g: 0xFF,
    b: 0xFF,
    a: 0xFF,
};
pub const RED_A: RGBA8 = RGBA8 {
    r: 0xFF,
    g: 0x00,
    b: 0x00,
    a: 0xFF,
};
pub const YELLOW_A: RGBA8 = RGBA8 {
    r: 0xFF,
    g: 0xFF,
    b: 0x00,
    a: 0xFF,
};
pub const LIME_A: RGBA8 = RGBA8 {
    r: 0x00,
    g: 0xFF,
    b: 0x00,
    a: 0xFF,
};
pub const BLUE_A: RGBA8 = RGBA8 {
    r: 0x00,
    g: 0x00,
    b: 0xFF,
    a: 0xFF,
};
pub const FUCHSIA_A: RGBA8 = RGBA8 {
    r: 0xFF,
    g: 0x00,
    b: 0xFF,
    a: 0xFF,
};
pub const ORANGE_A: RGBA8 = RGBA8 {
    r: 0xFF,
    g: 0xA5,
    b: 0x00,
    a: 0xFF,
};
pub const CHARTREUSE_A: RGBA8 = RGBA8 {
    r: 0x7F,
    g: 0xFF,
    b: 0x00,
    a: 0xFF,
};
pub const SPRING_GREEN_A: RGBA8 = RGBA8 {
    r: 0x00,
    g: 0xFF,
    b: 0x7F,
    a: 0xFF,
};
pub const CYAN_A: RGBA8 = RGBA8 {
    r: 0x00,
    g: 0xFF,
    b: 0xFF,
    a: 0xFF,
};
pub const DEEP_BLUE_A: RGBA8 = RGBA8 {
    r: 0,
    g: 127,
    b: 255,
    a: 255,
};
pub const BLUE_PURPLE_A: RGBA8 = RGBA8 {
    r: 127,
    g: 0,
    b: 255,
    a: 255,
};
pub const DARK_PURPLE_A: RGBA8 = RGBA8 {
    r: 255,
    g: 0,
    b: 127,
    a: 255,
};
pub const T_3000K_A: RGBA8 = RGBA8 {
    r: 255,
    g: 180,
    b: 107,
    a: 255,
};
pub const T_3500K_A: RGBA8 = RGBA8 {
    r: 255,
    g: 196,
    b: 137,
    a: 255,
};
pub const T_4000K_A: RGBA8 = RGBA8 {
    r: 255,
    g: 209,
    b: 163,
    a: 255,
};
pub const T_5000K_A: RGBA8 = RGBA8 {
    r: 255,
    g: 228,
    b: 206,
    a: 255,
};

// Use const generic rainbows to make iterable rainbows of various sizes. Rainbows contain a
// list of colors in order, which will be used by animations as a color rainbow.
pub type Rainbow<'a> = &'a [RGBA8];

pub const R_BLACK: Rainbow = &[BLACK_A];
pub const R_WHITE: Rainbow = &[WHITE_A];
pub const R_RED: Rainbow = &[RED_A];
pub const R_ORANGE: Rainbow = &[ORANGE_A];
pub const R_YELLOW: Rainbow = &[YELLOW_A];
pub const R_CHARTREUSE: Rainbow = &[CHARTREUSE_A];
pub const R_LIME: Rainbow = &[LIME_A];
pub const R_SPRING_GREEN: Rainbow = &[SPRING_GREEN_A];
pub const R_CYAN: Rainbow = &[CYAN_A];
pub const R_DEEP_BLUE: Rainbow = &[DEEP_BLUE_A];
pub const R_BLUE: Rainbow = &[BLUE_A];
pub const R_BLUE_PURPLE: Rainbow = &[BLUE_PURPLE_A];
pub const R_FUCHSIA: Rainbow = &[FUCHSIA_A];
pub const R_DARK_PURPLE: Rainbow = &[DARK_PURPLE_A];
pub const R_ROYGBIV: Rainbow = &[RED_A, YELLOW_A, LIME_A, CYAN_A, BLUE_A, FUCHSIA_A];
pub const R_RYB: Rainbow = &[RED_A, BLACK_A, YELLOW_A, BLACK_A, BLUE_A, BLACK_A];
pub const R_OGP: Rainbow = &[ORANGE_A, BLACK_A, LIME_A, BLACK_A, FUCHSIA_A, BLACK_A];
pub const R_RGB: Rainbow = &[RED_A, BLACK_A, LIME_A, BLACK_A, BLUE_A, BLACK_A];
pub const R_BY: Rainbow = &[BLUE_A, BLACK_A, YELLOW_A, BLACK_A];
pub const R_RB: Rainbow = &[RED_A, BLACK_A, CYAN_A, BLACK_A];
pub const R_OB: Rainbow = &[ORANGE_A, BLACK_A, DEEP_BLUE_A, BLACK_A];
pub const R_BW: Rainbow = &[BLUE_A, BLACK_A, WHITE_A, BLACK_A];
pub const R_RW: Rainbow = &[RED_A, BLACK_A, WHITE_A, BLACK_A];
pub const R_GW: Rainbow = &[LIME_A, BLACK_A, WHITE_A, BLACK_A];

pub const fn dark_pattern(base: RGBA8) -> [RGBA8; 6] {
    let mut colors = [BLACK_A; 6];
    let mut i = 0;
    while i < 3 {
        colors[i * 2] = RGBA8 {
            r: base.r / 2,
            g: base.g / 2,
            b: base.b / 2,
            a: 255,
        };
        colors[i * 2 + 1] = RGBA8 {
            r: base.r / 4,
            g: base.g / 4,
            b: base.b / 4,
            a: 255,
        };
        i += 1;
    }
    colors
}

pub const R_DARK_RED_PATTERN: Rainbow = &dark_pattern(RED_A);
pub const R_DARK_YELLOW_PATTERN: Rainbow = &dark_pattern(YELLOW_A);
pub const R_DARK_GREEN_PATTERN: Rainbow = &dark_pattern(LIME_A);
pub const R_DARK_SKY_BLUE_PATTERN: Rainbow = &dark_pattern(CYAN_A);
pub const R_DARK_BLUE_PATTERN: Rainbow = &dark_pattern(BLUE_A);
pub const R_DARK_PURPLE_PATTERN: Rainbow = &dark_pattern(FUCHSIA_A);
pub const R_WHITE_PATTERN: Rainbow = &dark_pattern(WHITE_A);
pub const R_VU_METER: Rainbow = &[
    LIME_A, LIME_A, LIME_A, LIME_A, LIME_A, LIME_A, LIME_A, YELLOW_A, YELLOW_A, RED_A,
];

pub const NUM_RAINBOWS: usize = 31;

/// This is an array of the rainbow consts above that can be used to cycle through rainbows in animations.
pub const RAINBOW_ARRAY: [&[RGBA8]; NUM_RAINBOWS] = [
    R_BLACK,
    R_WHITE,
    R_RED,
    R_ORANGE,
    R_YELLOW,
    R_CHARTREUSE,
    R_LIME,
    R_SPRING_GREEN,
    R_CYAN,
    R_DEEP_BLUE,
    R_BLUE,
    R_BLUE_PURPLE,
    R_FUCHSIA,
    R_DARK_PURPLE,
    R_ROYGBIV,
    R_RYB,
    R_OGP,
    R_RGB,
    R_BY,
    R_RB,
    R_OB,
    R_BW,
    R_RW,
    R_GW,
    R_DARK_RED_PATTERN,
    R_DARK_YELLOW_PATTERN,
    R_DARK_GREEN_PATTERN,
    R_DARK_SKY_BLUE_PATTERN,
    R_DARK_BLUE_PATTERN,
    R_DARK_PURPLE_PATTERN,
    R_WHITE_PATTERN,
];
