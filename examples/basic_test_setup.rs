use embedded_time::rate::Extensions;
use lc::animations::{Animatable, Animation};
use lc::{default_animations, LightingController, LogicalStrip};
use lighting_controller as lc;
use lighting_controller::colors::BLACK_A;

fn main() {
    let frame_rate = 60.Hz();
    let color_buffer = &mut [BLACK_A; 16];
    let mut ls = LogicalStrip::new(color_buffer);
    let a1 = &mut Animation::<16>::new(default_animations::ANI_TEST, frame_rate);
    let animations: [&mut dyn Animatable; 1] = [a1];
    let mut lc = LightingController::new(animations, frame_rate);

    loop {
        //this should have a check to limit updates to match the frame_rate:
        lc.update(&mut ls);
    }
}
