use crate::colors::ManipulatableColor;
use crate::{
    animations::{Direction, MAX_OFFSET},
    colors::Rainbow,
};
use core::ops::Index;
use embedded_time::rate::*;
use rgb::RGB8;

pub fn convert_ns_to_frames(nanos: u64, frame_rate: Hertz) -> usize {
    (nanos * frame_rate.integer() as u64 / 1_000_000_000_u64) as usize
}

pub fn convert_ms_to_frames(millis: u64, frame_rate: Hertz) -> usize {
    (millis * frame_rate.integer() as u64 / 1_000_u64) as usize
}

/// Returns a translation array beginning with index `start_at` and
/// incrementing until reaching the desired `SIZE`
pub fn default_translation_array<const SIZE: usize>(start_at: usize) -> [usize; SIZE] {
    let mut result: [usize; SIZE] = [0; SIZE];
    for (index, value) in result.iter_mut().enumerate() {
        *value = start_at + index;
    }
    result
}

/// TODO: Implement an actual random function that's not hardware dependent here
pub fn get_random_offset() -> u16 {
    12
}

pub fn shift_offset(starting_offset: u16, frames: Progression, direction: Direction) -> u16 {
    if frames.total == 0 {
        return starting_offset;
    }
    let max_offset = MAX_OFFSET as usize;
    let starting_offset = starting_offset as usize;
    let offset_shift = match direction {
        Direction::Positive => max_offset * frames.get_current() / frames.total,
        Direction::Negative => max_offset * (frames.total - frames.get_current()) / frames.total,
        Direction::Stopped => 0,
    };
    (starting_offset + offset_shift) as u16
}

pub struct ReversibleRainbow<'a> {
    backer: Rainbow<'a>,
    is_forward: bool,
}

impl<'a> ReversibleRainbow<'a> {
    pub fn len(&self) -> usize {
        self.backer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.backer.is_empty()
    }
}

impl<'a> Index<usize> for ReversibleRainbow<'a> {
    type Output = RGB8;

    fn index(&self, index: usize) -> &Self::Output {
        match self.is_forward {
            true => &self.backer[index],
            false => &self.backer[self.backer.len() - 1 - index],
        }
    }
}

pub trait FadeRainbow {
    fn rainbow(&self) -> &StatefulRainbow;
    fn frames(&self) -> &Progression;

    fn calculate_fade_color(&self) -> RGB8 {
        let (rainbow, frames) = (self.rainbow(), self.frames());

        let current_color = rainbow.current_color();
        if frames.total == 0 {
            return current_color;
        }
        let next_color = rainbow.peek_next_color();
        current_color.lerp_with(next_color, *frames)
    }

    fn current_fade_color(&self) -> RGB8 {
        self.rainbow().current_color()
    }
}

pub trait MarchingRainbow {
    fn rainbow(&self) -> &StatefulRainbow;
    fn frames(&self) -> &Progression;

    fn current_rainbow_color(&self) -> RGB8 {
        self.rainbow().current_color()
    }
}

pub trait MarchingRainbowMut<'a> {
    fn rainbow_mut(&mut self) -> &mut StatefulRainbow<'a>;
    fn frames_mut(&mut self) -> &mut Progression;

    /// Advances the rainbow color and resets the frame count
    fn advance_rainbow_color(&mut self) {
        self.rainbow_mut().increment();
        self.frames_mut().reset();
    }
}

pub struct TimedRainbows<'a, 'b> {
    pub fade_rainbow: &'b mut StatefulRainbow<'a>,
    pub incremental_rainbow: &'b mut StatefulRainbow<'a>,
    pub frames: &'b mut Progression,
}

impl<'a, 'b> FadeRainbow for TimedRainbows<'a, 'b> {
    fn rainbow(&self) -> &StatefulRainbow {
        self.fade_rainbow
    }
    fn frames(&self) -> &Progression {
        self.frames
    }
}

impl<'a, 'b> MarchingRainbow for TimedRainbows<'a, 'b> {
    fn rainbow(&self) -> &StatefulRainbow {
        self.incremental_rainbow
    }
    fn frames(&self) -> &Progression {
        self.frames
    }
}

impl<'a, 'b> MarchingRainbowMut<'a> for TimedRainbows<'a, 'b> {
    fn rainbow_mut(&mut self) -> &mut StatefulRainbow<'a> {
        self.incremental_rainbow
    }
    fn frames_mut(&mut self) -> &mut Progression {
        self.frames
    }
}

pub struct StatefulRainbow<'a> {
    pub backer: ReversibleRainbow<'a>,
    pub position: Progression,
}

impl<'a> StatefulRainbow<'a> {
    pub fn new(rainbow: &'a [RGB8], is_forward: bool) -> StatefulRainbow<'a> {
        let position = Progression::new(rainbow.len());
        let backer = ReversibleRainbow {
            backer: rainbow,
            is_forward,
        };
        Self { backer, position }
    }

    pub fn current_color(&self) -> RGB8 {
        self.backer[self.position.get_current() as usize]
    }

    pub fn decrement(&mut self) {
        self.position.decrement();
    }

    pub fn increment(&mut self) {
        self.position.increment();
    }

    pub fn peek_next_color(&self) -> RGB8 {
        self.backer[self.position.peek_next() as usize]
    }

    pub fn peek_last_color(&self) -> RGB8 {
        self.backer[self.position.peek_prev() as usize]
    }

    pub fn reset(&mut self) {
        self.position.reset();
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Progression {
    current: usize,
    pub total: usize,
    pub is_forward: bool,
}

impl Progression {
    pub fn new(total: usize) -> Self {
        Self {
            current: 0,
            total,
            is_forward: true,
        }
    }

    pub fn reverse_direction(&mut self) {
        self.is_forward = !self.is_forward;
    }

    fn is_mono(&self) -> bool {
        self.total <= 1
    }

    pub fn is_first_frame(&self) -> bool {
        self.current == 0
    }

    pub fn get_current(&self) -> usize {
        if self.is_mono() {
            return 0;
        }
        match self.is_forward {
            true => self.current,
            false => self.total - 1 - self.current,
        }
    }

    pub fn set_current(&mut self, value: usize) {
        if self.is_mono() {
            return;
        }
        let value = value % self.total;
        self.current = value;
    }

    pub fn decrement(&mut self) {
        if self.is_mono() {
            return;
        }
        self.current = self.peek_prev();
    }

    pub fn checked_decrement(&mut self) -> bool {
        if self.is_mono() {
            return false;
        }
        self.decrement();
        self.current == self.total - 1
    }

    pub fn increment(&mut self) {
        if self.is_mono() {
            return;
        }
        self.current = self.peek_next();
    }

    pub fn checked_increment(&mut self) -> bool {
        if self.is_mono() {
            return false;
        }
        self.increment();
        self.current == 0
    }

    pub fn peek_next(&self) -> usize {
        self.up_one()
    }

    pub fn peek_prev(&self) -> usize {
        self.down_one()
    }

    fn up_one(&self) -> usize {
        if self.is_mono() {
            return 0;
        }
        (self.current + 1) % self.total
    }

    fn down_one(&self) -> usize {
        if self.is_mono() {
            return 0;
        }
        (self.current + self.total - 1) % self.total
    }

    pub fn reset(&mut self) {
        self.current = 0
    }
}
