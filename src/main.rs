use std::path::PathBuf;

use clap::{arg, command, value_parser};
use image::{ImageBuffer, Rgb, RgbImage};
use tinyrenderer_rs::{grid::Grid, obj::Obj, Vec3};

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let matches = command!()
        .arg(
            arg!(
            --file <FILE>)
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let file_path = matches.get_one::<PathBuf>("file").expect("required");

    if file_path
        .extension()
        .expect("needs to be a file with an obj extension")
        .to_str()
        .unwrap()
        != "obj"
    {
        eprintln!("needs to be a file with an obj extension");
        std::process::exit(1);
    }

    let model = Obj::from(file_path).unwrap();

    // println!("{:#?}", model)

    // model.render_wireframe(&mut img, red);

    // let p1: Vec3<isize> = Vec3::new(7, 45, 0);
    // let p2: Vec3<isize> = Vec3::new(35, 100, 0);
    // let p3: Vec3<isize> = Vec3::new(45, 60, 0);
    // let p4: Vec3<isize> = Vec3::new(120, 35, 0);
    // let p5: Vec3<isize> = Vec3::new(90, 5, 0);
    // let p6: Vec3<isize> = Vec3::new(45, 110, 0);
    // let p7: Vec3<isize> = Vec3::new(115, 83, 0);
    // let p8: Vec3<isize> = Vec3::new(80, 90, 0);
    // let p9: Vec3<isize> = Vec3::new(85, 120, 0);

    let mut grid: Grid<[u8; 3], WIDTH, HEIGHT> = Default::default();

    model.render(&mut grid);

    // draw_triangle(p1, p2, p3, &mut grid);
    // draw_triangle(p4, p5, p6, &mut grid);
    // draw_triangle(p7, p8, p9, &mut grid);

    let mut img_buf: RgbImage = ImageBuffer::new(grid.width() as u32, grid.height() as u32);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let color = grid.get(&Vec3::new(x as usize, y as usize, 1));
        *pixel = Rgb(*color);
    }

    image::imageops::rotate180_in_place(&mut img_buf);

    img_buf.save("output/test.png").unwrap();
}
