use std::{
    borrow::{Borrow, BorrowMut},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use image::{Rgb, RgbImage};

use crate::{draw_line, Point};

#[derive(Debug)]
pub struct Obj {
    points: Vec<Point<f32>>,
    faces: Vec<Vec<usize>>,
}

impl Obj {
    pub fn from<P>(filename: P) -> Option<Obj>
    where
        P: AsRef<Path>,
    {
        if let Ok(lines) = read_lines(filename) {
            // Obj files indices start at 1, so create a dummy entry to avoid instantiating later
            let mut points: Vec<Point<f32>> = vec![];

            let mut faces: Vec<Vec<usize>> = vec![];

            for line in lines.map_while(anyhow::Result::ok) {
                match line {
                    s if s.starts_with("v ") => points.push(string_to_point(s[2..].to_string())),
                    s if s.starts_with("f ") => faces.push(string_to_face(s[2..].to_string())),

                    _ => {}
                }
            }
            Some(Obj { points, faces })
        } else {
            None
        }
    }

    pub fn render(self, image: &mut RgbImage, colour: Rgb<u8>) {
        println!("242: {}", self.points[242].y());
        for i in 0..self.faces.len() {
            let face = self.faces[i].clone();
            for j in 0..3 {
                let v0 = self.points[face[j]];
                let v1 = self.points[face[(j + 1) % 3]];
                let x0 = ((v0.x() + 1.) * (image.width() as f32) / 2.) as isize;
                let y0 = ((v0.y() + 1.) * (image.height() as f32) / 2.) as isize;
                let x1 = ((v1.x() + 1.) * (image.width() as f32) / 2.) as isize;
                let y1 = ((v1.y() + 1.) * (image.height() as f32) / 2.) as isize;
                // let x0 = ((v0.x() + 1.) * (image.width() as f32) / 2.)
                //     .clamp(0.0, (image.width() - 1) as f32) as isize;
                // let y0 = ((v0.y() + 1.) * (image.height() as f32) / 2.)
                //     .clamp(0.0, (image.height() - 1) as f32) as isize;
                // let x1 = ((v1.x() + 1.) * (image.width() as f32) / 2.)
                //     .clamp(0.0, (image.width() - 1) as f32) as isize;
                // let y1 = ((v1.y() + 1.) * (image.height() as f32) / 2.)
                //     .clamp(0.0, (image.height() - 1) as f32) as isize;
                // println!("indice1: {}; indice2: {}", face[j], face[(j + 1) % 3]);
                // println!("x0: {}; y0: {}; x1: {}; y1: {}", x0, y0, x1, y1);
                draw_line(x0, y0, x1, y1, image, colour)
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn string_to_point(line: String) -> Point<f32> {
    let splits: Vec<f32> = line
        .split_ascii_whitespace()
        .take(3)
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    Point(splits[0], splits[1], splits[2])
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
