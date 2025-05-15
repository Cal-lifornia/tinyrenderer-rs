use image::{Rgb, RgbImage};
use tinyrenderer_rs::{draw_line, obj::Obj, Point};

fn main() {
    let mut img = RgbImage::new(800, 800);

    // let blue = Rgb([64, 128, 255]);
    // let green = Rgb([0, 255, 0]);
    let red = Rgb([255, 0, 0]);
    // let yellow = Rgb([255, 200, 0]);

    let model = Obj::from("./obj/african_head/african_head.obj").unwrap();

    // println!("{:#?}", model)

    model.render(&mut img, red);

    img.save("./output.ppm").unwrap();
}
