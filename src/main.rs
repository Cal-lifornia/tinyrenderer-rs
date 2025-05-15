use std::{path::PathBuf, process};

use clap::{arg, command, value_parser};
use image::{Rgb, RgbImage};
use tinyrenderer_rs::{draw_triangle, obj::Obj, Point};

fn main() {
    // let matches = command!()
    //     .arg(
    //         arg!(
    //         --file <FILE>)
    //         .required(true)
    //         .value_parser(value_parser!(PathBuf)),
    //     )
    //     .get_matches();

    // let file_path = matches.get_one::<PathBuf>("file").expect("required");

    // if file_path
    //     .extension()
    //     .expect("needs to be a file with an obj extension")
    //     .to_str()
    //     .unwrap()
    //     != "obj"
    // {
    //     eprintln!("needs to be a file with an obj extension");
    //     std::process::exit(1);
    // }

    let mut img = RgbImage::new(128, 128);

    // let blue = Rgb([64, 128, 255]);
    let green = Rgb([0, 255, 0]);
    let red = Rgb([255, 0, 0]);
    let yellow = Rgb([255, 200, 0]);

    // let model = Obj::from(file_path).unwrap();

    // println!("{:#?}", model)

    // model.render_wireframe(&mut img, red);

    let p1: Point<isize> = Point::new(7, 45, 0);
    let p2: Point<isize> = Point::new(35, 100, 0);
    let p3: Point<isize> = Point::new(45, 60, 0);
    let p4: Point<isize> = Point::new(120, 35, 0);
    let p5: Point<isize> = Point::new(90, 5, 0);
    let p6: Point<isize> = Point::new(45, 110, 0);
    let p7: Point<isize> = Point::new(115, 83, 0);
    let p8: Point<isize> = Point::new(80, 90, 0);
    let p9: Point<isize> = Point::new(85, 120, 0);

    draw_triangle(p1, p2, p3, &mut img, green);
    draw_triangle(p4, p5, p6, &mut img, red);
    draw_triangle(p7, p8, p9, &mut img, yellow);

    img.save("./output.png").unwrap();
}
