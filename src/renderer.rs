use std::mem::swap;

use rayon::prelude::*;

use crate::{
    grid::{Grid, Point},
    signed_triangle_area, Vec3,
};

// pub struct Renderer {
//     pub filename: &'static str,
//     pub output_dir: &'static str,
// }

// pub fn draw_triangle<const W: usize, const H: usize>(
//     v1: Vec3<isize>,
//     v2: Vec3<isize>,
//     v3: Vec3<isize>,
//     pixels: &mut Grid<[u8; 3], W, H>,
//     colour: [u8; 3],
// ) {
//     let total_area = signed_triangle_area(v1.x(), v1.y(), v2.x(), v2.y(), v3.x(), v3.y());
//     if total_area < 1.0 {
//         return;
//     }

//     pixels.set_all_parallel(calculate_pixel(total_area, v1, v2, v3, colour))
// }

pub fn calculate_pixel(
    total_area: f64,
    v1: Vec3<isize>,
    v2: Vec3<isize>,
    v3: Vec3<isize>,
    colour: [u8; 3],
    p: Point,
) -> Option<[u8; 3]> {
    let alpha = signed_triangle_area(p.into(), v2, v3) / total_area;
    let beta = signed_triangle_area(p.into(), v3, v1) / total_area;
    let gamma = signed_triangle_area(p.into(), v1, v2) / total_area;
    if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
        None
    } else {
        Some(colour)
    }
}

// if p1.y() > p2.y() {
//     swap(&mut p1, &mut p2);
// }
// if p1.y() > p3.y() {
//     swap(&mut p1, &mut p3);
// }

// if p2.y() > p3.y() {
//     swap(&mut p2, &mut p3);
// }

// let total_height = p3.y() - p1.y();
// if p1.y() != p2.y() {
//     let segment_height = p2.y() - p1.y();
//     // for y in p1.y()..=p2.y() {
//     //     let x1 = p1.x() + ((p3.x() - p1.x()) * (y - p1.y())) / total_height;
//     //     let x2 = p1.x() + ((p2.x() - p1.x()) * (y - p1.y())) / segment_height;

//     //     // image.put_pixel(x1 as u32, y as u32, RED);
//     //     // image.put_pixel(x2 as u32, y as u32, GREEN);
//     //     draw_line(x1, y, x2, y, image, GREEN)
//     // }
//     (p1.y()..=p2.y()).into_par_iter().for_each(|y| {
//         let x1 = p1.x() + ((p3.x() - p1.x()) * (y - p1.y())) / total_height;
//         let x2 = p1.x() + ((p2.x() - p1.x()) * (y - p1.y())) / segment_height;

//         // image.put_pixel(x1 as u32, y as u32, RED);
//         // image.put_pixel(x2 as u32, y as u32, GREEN);

//         for x in x1.min(x2)..x1.max(x2) {}
//     });
// }
// if p2.y() != p3.y() {
//     let segment_height = p3.y() - p2.y();
//     for y in p2.y()..=p3.y() {
//         let x1 = p1.x() + ((p3.x() - p1.x()) * (y - p1.y())) / total_height;
//         let x2 = p2.x() + ((p3.x() - p2.x()) * (y - p2.y())) / segment_height;
//         draw_line(x1, y, x2, y, image, GREEN);
//     }
// }

// draw_line(p1.x(), p1.y(), p2.x(), p2.y(), image, colour);
// draw_line(p2.x(), p2.y(), p3.x(), p3.y(), image, colour);
// draw_line(p3.x(), p3.y(), p1.x(), p1.y(), image, colour);
