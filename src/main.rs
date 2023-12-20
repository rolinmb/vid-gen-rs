use std::fs;
use std::path::Path;
//use std::env;
use image::{Rgba, RgbaImage};

fn cleandir(dir: &str) -> std::io::Result<()> {
  fs::remove_dir_all(dir)?;
  fs::create_dir_all(dir)?;
  Ok(())
}

fn gencleanup(pngdir: &str, pngname: &str, frames: u32) {
  for i in 1..(frames+1) {
    let idx = ((frames*2)-1)-(i-1);
    let pathstr = format!("{}/{}_{}.png", pngdir, pngname, idx);
    let pngpath = Path::new(&pathstr);
    if pngpath.exists() {
      match fs::remove_file(pngpath) {
        Ok(_) => println!("gencleanup(): Successfully removed '{}'", pngpath.display()),
        Err(err) => eprintln!("gencleanup(): Error removing '{}': {}", pngpath.display(), err),
      }
    } else {
      println!("gencleanup(): '{}' does not exist; nothing to remove", pngpath.display());
    }
  }
}

fn customgen(
  pngdir: &str, pngname: &str, vidname: &str,
  width: u32, height: u32, frames: u32,
  f_r: impl Fn(u32, u32) -> u32,
  f_g: impl Fn(u32, u32) -> u32,
  f_b: impl Fn(u32, u32) -> u32,
) {
  if !fs::metadata(pngdir).is_ok() {
    if let Err(err) = fs::create_dir_all(pngdir) {
      eprintln!("customgen(): Error creating directory '{}': {}", pngdir, err);
    } else {
      println!("customgen(): Successfully created '{}'", pngdir);
    }
  } else {
    println!("customgen(): '{}' already exists; cleaning directory contents...", pngdir);
    if let Err(err) = cleandir(pngdir) {
      eprintln!("Error cleaning '{}': {}", pngdir, err);
    }
    println!("customgen(): Successfully cleaned '{}'", pngdir);
  }
  let mut pngframe = RgbaImage::new(width, height);
  for i in 1..frames + 1 {
    for x in 0..width {
      for y in 0..height {
        pngframe.put_pixel(
          x, y,
          Rgba([f_r(x, y) as u8, f_g(x, y) as u8, f_b(x, y) as u8, 255]),
        );
      }
    }
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, i - 1 )).unwrap();
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, (((frames*2)-1)-(i-1)) )).unwrap();
  }
  //TODO: Run ffmpeg commands, reference vid-gen-go
  println!("\ncustomgen(): Successfully generated .png frames for .mp4\n\nTODO: Create {} with ffmpeg\n", vidname);
  gencleanup(pngdir, pngname, frames);
  println!("\ncustomgen(): Successfully cleaned up extra frames in '{}'", pngdir);
}

fn main() {
  println!("main(): Hello, world!");
  customgen(
    &String::from("src/png_out/test0"),
    &String::from("test0"),
    &String::from("test0"),
    1000,
    1000,
    30,
    |x, y| (x + y) as u32,
    |x, y| (x.wrapping_sub(y)) as u32,
    |x, y| (x * y) as u32,
  );
}
