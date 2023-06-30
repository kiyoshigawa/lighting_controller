#![no_std]

pub mod animations;
pub mod colors;
pub mod default_animations;
pub mod utility;

use crate::animations::{Animatable, AnimationType};
use crate::colors::ManipulatableColor;
use embedded_time::rate::Hertz;
use rgb::RGBA8;

pub struct LogicalStrip<'a> {
    pub color_buffer: &'a mut [RGBA8],
}

impl<'a> LogicalStrip<'a> {
    pub fn new(color_buffer: &'a mut [RGBA8]) -> Self {
        LogicalStrip { color_buffer }
    }

    pub fn get_color_at_index(&self, index: usize) -> RGBA8 {
        self.color_buffer[index]
    }

    // this sets the color value in the color array at index:
    pub fn set_color_at_index(&mut self, index: usize, color: RGBA8) {
        self.color_buffer[index].set_color(color);
    }

    // this fills the entire strip with a single color:
    pub fn set_strip_to_solid_color(&mut self, color: RGBA8) {
        for c in &mut self.color_buffer.iter_mut() {
            c.set_color(color);
        }
    }
}

pub struct LightingController<'a, const N_ANI: usize> {
    animations: [&'a mut dyn Animatable<'a>; N_ANI],
    frame_rate: Hertz,
}

impl<'a, const N_ANI: usize> LightingController<'a, N_ANI> {
    pub fn new(
        animations: [&'a mut dyn Animatable<'a>; N_ANI],
        frame_rate: impl Into<Hertz>,
    ) -> Self {
        let frame_rate = frame_rate.into();

        LightingController {
            animations,
            frame_rate,
        }
    }

    pub fn update(&mut self, logical_strip: &mut LogicalStrip) {
        for animation in self.animations.iter_mut() {
            animation.update();

            let segment = animation.segment();
            let translater = animation.translation_array();
            let translated = translater.iter().zip(segment.iter());

            for (&index, &color) in translated {
                logical_strip.set_color_at_index(index, color);
            }
        }
    }

    pub fn trigger(&mut self, animation_index: usize, params: &animations::trigger::Parameters) {
        self.animations[animation_index].trigger(params, self.frame_rate);
    }

    pub fn set_offset(&mut self, animation_index: usize, a_type: AnimationType, offset: u16) {
        self.animations[animation_index].set_offset(a_type, offset);
    }

    pub fn replace_animation(&mut self, index: usize, new_anim: &'a mut dyn Animatable<'a>) {
        self.animations[index] = new_anim;
    }
}
