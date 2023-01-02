//! A module holding the various types of graphs and charts

pub mod scatter;

use image::{Rgb, Rgba};
use rand::Rng;

pub fn random_rgb() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let (r, g, b) = (rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
    Rgb([r, g, b])
}

pub fn random_rgba(alpha: u8) -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    let (r, g, b) = (rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255));
    Rgba([r, g, b, alpha])
}