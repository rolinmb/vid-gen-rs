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
              ((fr_theta(xfloat,yfloat) * scale * f_r(xfloat,yfloat)) % 256.0) as u8,
              ((fg_theta(xfloat,yfloat) * scale * f_g(xfloat,yfloat)) % 256.0) as u8,
              ((fb_theta(xfloat,yfloat) * scale * f_b(xfloat,yfloat)) % 256.0) as u8,
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
  println!("\ngenclosures(): Successfully executed ffmpeg command to generate {}", vidname);
  gencleanup(pngdir, pngname, frames);
  println!("\ngenclosures(): Successfully cleaned up extra frames in '{}'", pngdir);
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
    |x, y| (((x*y).cos()) * (x*x - y*y)) as f64, // fb_theta()
    |x, y| (((x*y).tan()) * (x-y)) as f64, // fg_theta()
  );
}
