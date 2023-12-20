use std::env;
use image::{GenericImageView, Rgba, RgbaImage};

fn customgen(
  pngdir: &str, pngname: &str, vidname: &str,
  width: u32, height: u32,
  f_r: impl Fn(u32, u32) -> u32,
  f_g: impl Fn(u32, u32) -> u32,
  f_b: impl Fn(u32, u32) -> u32,
) {
  let mut pngframe = RgbaImage::new(width, height);
  for i in 1..frames + 1 {
    for x in 0..width {
      for y in 0..height {
        newpng.put_pixel(
          x, y,
          Rgba([f_r(x, y), f_g(x, y), f_b(x, y), 255]),
        );
      }
    }
    pngframe.save(format!("src/png_out/{}/{}_{}.png", pngdir, pngname, i - 1 )).unwrap();
    pngframe.save(format!("src/png_out/{}/{}_{}.png", pngdir, pngname, (((frames*2)-1)-(i-1)) )).unwrap();
  }
  //TODO: Run ffmpeg commands, reference vid-gen-go
}

fn main() {
  println!("Hello, world!");
  customgen(
    &String::from("test0"),
    &String::from("test0"),
    &String::from("test0"),
    1000,
    1000,
    |x, y| (x + y) as u32,
    |x, y| (x - y) as u32,
    |x, y| (x * y) as u32,
  );
}
