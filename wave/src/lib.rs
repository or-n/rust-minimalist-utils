use num::{scale::*, ratio::max::*, operation::smoothstep::*};
use array::samples::*;

pub const C: f32 = 261.63;
pub const E: f32 = 329.63;
pub const G: f32 = 392.0;
pub const A: f32 = 440.0;

pub const DEFAULT_SAMPLE_RATE: u32 = 44100;

pub fn fade(delta: impl Fn(RatioIndex) -> usize, duration: usize)
-> impl Fn(f32, RatioIndex) -> f32 {
    move |x, i| {
        let delta = clamp_max(delta(i), duration);
        x.scale(smoothstep(delta))
    }
}