use std::fs;
use std::path::Path;
use std::process::Command;
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
      println!("\ngencleanup(): '{}' does not exist; nothing to remove", pngpath.display());
    }
  }
}

fn genclosures(
  pngdir: &str, pngname: &str, vidname: &str,
  width: u32, height: u32, frames: u32,
  mut scale: f64, scalefactor: f64,
  f_r: impl Fn(f64, f64) -> f64,
  f_g: impl Fn(f64, f64) -> f64,
  f_b: impl Fn(f64, f64) -> f64,
  fr_theta: impl Fn(f64, f64) -> f64,
  fg_theta: impl Fn(f64, f64) -> f64,
  fb_theta: impl Fn(f64, f64) -> f64
) {
  if !fs::metadata(pngdir).is_ok() {
    if let Err(err) = fs::create_dir_all(pngdir) {
      eprintln!("genclosures(): Error creating directory '{}': {}", pngdir, err);
    } else {
      println!("\ngenclosures(): Successfully created '{}'", pngdir);
    }
  } else {
    println!("genclosures(): '{}' already exists; cleaning directory contents...", pngdir);
    if let Err(err) = cleandir(pngdir) {
      eprintln!("genclojures(): Error cleaning '{}': {}", pngdir, err);
    }
    println!("\ngenclosures(): Successfully cleaned '{}'", pngdir);
  }
  let mut pngframe = RgbaImage::new(width, height);
  for i in 1..frames+1 {
    for x in 0..width {
      for y in 0..height {
        let xfloat = x as f64;
        let yfloat = y as f64;
        pngframe.put_pixel(
          x, y,
          Rgba(
            [
              ((fr_theta(xfloat,yfloat)*scale*f_r(xfloat,yfloat)) % 256.0) as u8,
              ((fg_theta(xfloat,yfloat)*scale*f_g(xfloat,yfloat)) % 256.0) as u8,
              ((fb_theta(xfloat,yfloat)*scale*f_b(xfloat,yfloat)) % 256.0) as u8,
              255,
            ],
          ),
        );
      }
    }
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, i-1 )).unwrap();
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, (((frames*2)-1)-(i-1)) )).unwrap();
    scale *= scalefactor
  }
  println!("\ngenclosures(): Successfully generated .png frames for .mp4");
  let _cmdffmpeg = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("ffmpeg -y -framerate 30 -i {}/{}_%d.png -c:v libx264 -pix_fmt yuv420p {}.mp4", pngdir, pngname, vidname)])
      .output()
      .expect("genclosures(): Failed to execute ffmpeg command")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("genclosures(): Failed to execute ffmpeg command")
  };
  println!("\ngenclosures(): Successfully executed ffmpeg command to generate {}.mp4", vidname);
  gencleanup(pngdir, pngname, frames);
  println!("\ngenclosures(): Successfully cleaned up extra frames in '{}'", pngdir);
}

fn genoverlay(
  pngname: &str, framesdir: &str, framename: &str, vidname: &str,
  frames: u32,
  ifactor: f64, mut scale: f64, scalefactor: f64,
  f_r: impl Fn(f64, f64) -> f64,
  f_g: impl Fn(f64, f64) -> f64,
  f_b: impl Fn(f64, f64) -> f64,
  fr_theta: impl Fn(f64, f64) -> f64,
  fg_theta: impl Fn(f64, f64) -> f64,
  fb_theta: impl Fn(f64, f64) -> f64
) {
  let pngimg = image::open(pngname.to_owned()+".png").expect(&format!("genoverlay(): could not find source image '{}.png' to use in overlay generation", pngname));
  let pngsrc: RgbaImage = pngimg.into_rgba8();
  if !fs::metadata(framesdir).is_ok() {
    if let Err(err) = fs::create_dir_all(framesdir) {
      eprintln!("genoverlay(): Error creating directory '{}': {}", framesdir, err);
    } else {
      println!("\ngenoverlay(): Successfully created '{}'", framesdir);
    }
  } else {
    println!("genoverlay(): '{}' already exists; cleaning directory contents...", framesdir);
    if let Err(err) = cleandir(framesdir) {
      eprintln!("genclojures(): Error cleaning '{}': {}", framesdir, err);
    }
    println!("\ngenoverlay(): Successfully cleaned '{}'", framesdir);
  }
  let mut pngframe = RgbaImage::new(pngsrc.width(), pngsrc.height());
  for i in 1..frames+1 {
    for x in 0..pngsrc.width() {
      for y in 0..pngsrc.height() {
        let pxlsrc: Rgba<u8> = *pngsrc.get_pixel(x, y);
        let xfloat = x as f64;
        let yfloat = y as f64;
        let r: f64 = (ifactor*(pxlsrc[0] as f64) + (1.0-ifactor)*(fr_theta(xfloat,yfloat)*scale*f_r(xfloat,yfloat))) % 256.0;
        let g: f64 = (ifactor*(pxlsrc[1] as f64) + (1.0-ifactor)*(fg_theta(xfloat,yfloat)*scale*f_g(xfloat,yfloat))) % 256.0;
        let b: f64 = (ifactor*(pxlsrc[2] as f64) + (1.0-ifactor)*(fb_theta(xfloat,yfloat)*scale*f_b(xfloat,yfloat))) % 256.0;
        pngframe.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, 255]));
      }
    }
    pngframe.save(format!("{}/{}_{}.png", framesdir, framename, i-1 )).unwrap();
    pngframe.save(format!("{}/{}_{}.png", framesdir, framename, (((frames*2)-1)-(i-1)) )).unwrap();
    scale *= scalefactor;
  }
  println!("\ngenoverlay(): Successfully generated .png frames for .mp4");
  let _cmdffmpeg = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("ffmpeg -y -framerate 30 -i {}/{}_%d.png -c:v libx264 -pix_fmt yuv420p {}.mp4", framesdir, framename, vidname)])
      .output()
      .expect("genoverlay(): Failed to execute ffmpeg command")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("genoverlay(): Failed to execute ffmpeg command")
  };
  println!("\ngenoverlay(): Successfully executed ffmpeg command to generate {}.mp4", vidname);
  gencleanup(framesdir, framename, frames);
  println!("\ngenoverlay(): Successfully cleaned up extra frames in '{}'", framesdir);
}

fn main() {
  genclosures(
    &String::from("src/png_out/test2.0"), // pngdir
    &String::from("test2.0"), // pngname
    &String::from("src/vid_out/test2.0"), // vidname
    1000, 1000, // width, height
    30, // frames
    1.42, 1.125, // scale, scalefactor
    |x, y| (x-y) as f64, // f_r()
    |x, y| (x*y) as f64, // f_g()
    |x, y| (x*x+y*y) as f64, // f_b()
    |x, y| (((x*y).sin()) * (x*x + y*y)) as f64, // fr_theta()
    |x, y| (((x*y).cos()) * (x*x - y*y)) as f64, // fg_theta()
    |x, y| (((x*y).tan()) * (x-y)) as f64, // fb_theta()
  );
  genoverlay(
    &String::from("src/png_in/test2.0_7"), // pngname
    &String::from("src/png_out/test3.0"), // framesdir
    &String::from("test3.0"), // framename
    &String::from("src/vid_out/test3.0"), // vidname
    30, // frames
    0.5, 1.42, 1.125, // ifactor, scale, scalefactor
    |x, y| (((x*y).sin()) * (x*x + y*y)) as f64, // f_r()
    |x, y| (((x*y).cos()) * (x*x - y*y)) as f64, // f_g()
    |x, y| (((x*y).tan()) * (x-y)) as f64, // f_b()
    |x, y| (x-y) as f64, // fr_theta()
    |x, y| (x*y) as f64, // fg_theta()
    |x, y| (x*x+y*y) as f64, // fb_theta()
  );
}
