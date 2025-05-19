use std::{mem::swap, ops::Mul, usize};

use image::{ImageBuffer, Rgb, RgbImage};
use num_traits::Num;
use rayon::prelude::*;

pub mod grid;
pub mod obj;
pub mod renderer;

pub const BLUE: Rgb<u8> = Rgb([64, 128, 255]);
pub const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const YELLOW: Rgb<u8> = Rgb([255, 200, 0]);

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T: Num + Copy>(pub T, pub T, pub T);

impl<T: Num + Mul + Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }

    pub fn z(&self) -> T {
        self.2
    }
}

impl From<Vec3<f32>> for Vec3<isize> {
    fn from(value: Vec3<f32>) -> Self {
        Self(value.0 as isize, value.1 as isize, value.2 as isize)
    }
}

pub fn draw_line(
    mut ax: isize,
    mut ay: isize,
    mut bx: isize,
    mut by: isize,
    image: &mut RgbImage,
    colour: Rgb<u8>,
) {
    ay = (image.height() - ay as u32) as isize;
    by = (image.height() - by as u32) as isize;

    let steep: bool = (ax - bx).abs() < (ay - by).abs();
    if steep {
        swap(&mut ax, &mut ay);
        swap(&mut bx, &mut by);
    }

    // make it left to right
    if ax > bx {
        swap(&mut ax, &mut bx);
        swap(&mut ay, &mut by);
    }

    for x in ax..bx {
        let t = (x - ax) as f64 / (bx - ax) as f64;
        let y = (ay as f64 + (by - ay) as f64 * t).round();

        if steep {
            image.put_pixel(y as u32, x as u32, colour);
        } else {
            image.put_pixel(x as u32, y as u32, colour);
        }
    }
}

pub fn draw_dot(x: isize, y: isize, image: &mut RgbImage, colour: Rgb<u8>) {
    image.put_pixel(x as u32, y as u32, colour);
}
