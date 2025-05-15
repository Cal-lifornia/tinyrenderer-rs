use std::{fmt::Display, mem::swap, ops::Mul};

use image::{Rgb, RgbImage};
use num_traits::Num;

pub mod obj;

#[derive(Debug, Clone, Copy)]
pub struct Point<T: Num + Copy>(pub T, pub T, pub T);

impl<T: Num + Mul + Copy> Point<T> {
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

pub fn draw_line(
    mut ax: isize,
    mut ay: isize,
    mut bx: isize,
    mut by: isize,
    image: &mut RgbImage,
    colour: Rgb<u8>,
) {
    // let mut ax = 64 - p1.x();
    // let mut ay = image.height() as isize - p1.y();
    // let mut bx = 64 - p2.x();
    // let mut by = image.height() as isize - p2.y();

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
            image.put_pixel((y - 1.0) as u32, (x as f32 - 1.0) as u32, colour);
        } else {
            image.put_pixel((x as f32 - 1.0) as u32, (y - 1.0) as u32, colour);
        }
    }
}
