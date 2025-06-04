use std::sync::mpsc::Sender;

use rayon::prelude::*;

use crate::{
    boundingbox::BoundingBox,
    grid::GridPoint,
    vec3::{signed_triangle_area, Colour, Point3},
};

pub fn raster_triangle(
    p1: &Point3,
    p2: &Point3,
    p3: &Point3,
    total_area: f64,
    colour: &Colour,
    chan: Sender<(GridPoint, [u8; 3])>,
) {
    let bbox = BoundingBox::new(p1, p2, p3);
    (bbox.min_x..=bbox.max_x).into_par_iter().for_each(|x| {
        (bbox.min_y..=bbox.max_y).for_each(|y| {
            let point = Point3::new(x as f64, y as f64, 0.0);
            let alpha = signed_triangle_area(&point, p2, p3) / total_area;
            let beta = signed_triangle_area(&point, p3, p1) / total_area;
            let gamma = signed_triangle_area(&point, p1, p2) / total_area;
            if !(alpha < 0.0 || beta < 0.0 || gamma < 0.0) {
                chan.send((GridPoint { x, y }, colour.to_rgb())).unwrap();
            }
        });
    });
}
