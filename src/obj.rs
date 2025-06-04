use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    grid::{Grid, GridPoint},
    renderer::raster_triangle,
    vec3::{signed_triangle_area, Colour, Point3, Vec3},
};

#[derive(Debug)]
pub struct Obj {
    verts: Vec<Vec3>,
    faces: Vec<Vec<usize>>,
}

impl Obj {
    pub fn from<P>(filename: P) -> Option<Obj>
    where
        P: AsRef<Path>,
    {
        if let Ok(lines) = read_lines(filename) {
            // Obj files indices start at 1, so create a dummy entry to avoid instantiating later
            let mut points: Vec<Point3> = vec![];

            let mut faces: Vec<Vec<usize>> = vec![];

            for line in lines.map_while(anyhow::Result::ok) {
                match line {
                    s if s.starts_with("v ") => points.push(string_to_point(s[2..].to_string())),
                    s if s.starts_with("f ") => faces.push(string_to_face(s[2..].to_string())),

                    _ => {}
                }
            }
            Some(Obj {
                verts: points,
                faces,
            })
        } else {
            None
        }
    }

    // pub fn render_wireframe(self, image: &mut RgbImage, colour: Rgb<u8>) {
    //     for i in 0..self.faces.len() {
    //         let face = self.faces[i].clone();
    //         for j in 0..3 {
    //             let v0 = self.verts[face[j]];
    //             let v1 = self.verts[face[(j + 1) % 3]];
    //             let x0 = ((v0.x() + 1.) * (image.width() as f32) / 2.)
    //                 .clamp(0.0, (image.width() - 1) as f32) as isize;
    //             let y0 = ((v0.y() + 1.) * (image.height() as f32) / 2.)
    //                 .clamp(0.0, (image.height() - 1) as f32) as isize;
    //             let x1 = ((v1.x() + 1.) * (image.width() as f32) / 2.)
    //                 .clamp(0.0, (image.width() - 1) as f32) as isize;
    //             let y1 = ((v1.y() + 1.) * (image.height() as f32) / 2.)
    //                 .clamp(0.0, (image.height() - 1) as f32) as isize;
    //             // println!("x0: {}; y0: {}; x1: {}; y1: {}", x0, y0, x1, y1);
    //             draw_line(x0, y0, x1, y1, image, colour);
    //             draw_dot(x0, y0, image, Rgb([255, 255, 255]));
    //             draw_dot(x1, y1, image, Rgb([255, 255, 255]));
    //         }
    //     }
    // }
    // pub fn render<const W: usize, const H: usize>(mut self, pixels: &mut Grid<[u8; 3], W, H>) {
    //     self.scale_all_points_parallel(pixels.width() as f64, pixels.height() as f64, 1000.0);
    //     for face in self.faces {
    //         let v1: Vec3<isize> = self.verts[face[0]].into();
    //         let v2: Vec3<isize> = self.verts[face[1]].into();
    //         let v3: Vec3<isize> = self.verts[face[2]].into();

    //         // let v1: Vec3<isize> = self.verts[face[0]].scale(pixels.width() as f32, pixels.height() as f32, 0.0).into();
    //         // let v2: Vec3<isize> = self.verts[face[1]].scale(pixels.width() as f32, pixels.height() as f32, 0.0).into();
    //         // let v3: Vec3<isize> = self.verts[face[2]].scale(pixels.width() as f32, pixels.height() as f32, 0.0).into();
    //         let total_area = signed_triangle_area(v1, v2, v3);

    //         if total_area < 1.0 {
    //             continue;
    //         }

    //         let mut rando = rand::rng();
    //         let colour: [u8; 3] = [
    //             rando.random_range(0..255),
    //             rando.random_range(0..255),
    //             rando.random_range(0..255),
    //         ];

    //         pixels.set_all_parallel(calculate_pixel(total_area, v1, v2, v3, colour));
    //     }
    // }
    pub fn render<const W: usize, const H: usize>(mut self, pixels: &mut Grid<[u8; 3], W, H>) {
        self.scale_all_points_parallel(pixels.width() as f64, pixels.height() as f64, 1000.0);
        let (sender, receiver): (Sender<(GridPoint, [u8; 3])>, Receiver<(GridPoint, [u8; 3])>) =
            mpsc::channel();
        self.faces.par_iter().for_each(|face| {
            let p1: Vec3 = self.verts[face[0]];
            let p2: Vec3 = self.verts[face[1]];
            let p3: Vec3 = self.verts[face[2]];

            let total_area = signed_triangle_area(&p1, &p2, &p3);

            if total_area >= 1.0 {
                let colour = Colour::random_real();
                raster_triangle(&p1, &p2, &p3, total_area, &colour, sender.clone());
            }
        });

        while let Ok((point, colour)) = receiver.try_recv() {
            pixels.set(point, colour);
        }
    }

    fn scale_all_points_parallel(&mut self, width: f64, height: f64, depth: f64) {
        use rayon::prelude::*;
        self.verts.par_iter_mut().for_each(|val| {
            *val = val.scale_up(width, height, depth);
        })
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn string_to_point(line: String) -> Vec3 {
    let splits: Vec<f64> = line
        .split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    Vec3::new(splits[0], splits[1], splits[2])
}

fn string_to_face(line: String) -> Vec<usize> {
    let splits: Vec<usize> = line
        .split_ascii_whitespace()
        .take(3)
        .map_while(|x| {
            let i = x.split("/").next().unwrap();
            i.parse::<usize>().ok()
        })
        .collect();
    vec![splits[0] - 1, splits[1] - 1, splits[2] - 1]
}
