use image::Rgb;

pub mod boundingbox;
pub mod camera;
pub mod grid;
pub mod obj;
pub mod renderer;
pub mod util;
pub mod vec3;

pub const BLUE: Rgb<u8> = Rgb([64, 128, 255]);
pub const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const YELLOW: Rgb<u8> = Rgb([255, 200, 0]);
