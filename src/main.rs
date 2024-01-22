use std::fs;
use std::path::Path;
use std::process::Command;
use image::{GrayImage, Rgba, RgbaImage, imageops};

const SOBELHORIZ: [f32; 9] = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
const SOBELVERTI: [f32; 9] = [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];

fn pngedges(srcpng: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> RgbaImage {
  let graypng = GrayImage::from_raw(srcpng.width(), srcpng.height(), srcpng.clone().into_raw()).unwrap();
  let gradx = imageops::filter3x3(&graypng, &SOBELHORIZ);
  let grady = imageops::filter3x3(&graypng, &SOBELVERTI);
  let mut edges = RgbaImage::new(srcpng.width(), srcpng.height());
  for x in 0..srcpng.width() {
    for y in 0..srcpng.height() {
      let magx = gradx.get_pixel(x, y)[0] as f32;
      let magy = grady.get_pixel(x, y)[0] as f32;
      let mag = (magx.powi(2) + magy.powi(2)).sqrt() as u8;
      edges.put_pixel(x, y, Rgba([mag, mag, mag, 255]));
    }
  }
  edges
}

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
        Ok(_) => {
          //println!("gencleanup(): Successfully removed '{}'", pngpath.display())
        }
        Err(err) => {
          eprintln!("gencleanup(): Error removing '{}': {}", pngpath.display(), err)
        }
      }
    } else {
      println!("\ngencleanup(): '{}' does not exist; nothing to remove", pngpath.display());
    }
  }
}

fn genclosures(
  pngdir: &str, pngname: &str, vidname: &str,
  width: u32, height: u32, frames: u32,
  mut scale: f64, scale_mult: f64,
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
    println!("\ngenclosures(): '{}' already exists; cleaning directory contents...", pngdir);
    if let Err(err) = cleandir(pngdir) {
      eprintln!("genclojures(): Error cleaning '{}': {}", pngdir, err);
    }
    println!("\ngenclosures(): Successfully cleaned '{}'", pngdir);
  }
  let mut pngframe = RgbaImage::new(width, height);
  for i in 1..frames+1 {
    for x in 0..width {
      for y in 0..height {
        let xf = x as f64;
        let yf = y as f64;
        let r: f64 = (fr_theta(xf,yf) * scale * f_r(xf,yf)) % 255.0;
        let g: f64 = (fg_theta(xf,yf) * scale * f_g(xf,yf)) % 255.0;
        let b: f64 = (fb_theta(xf,yf) * scale * f_b(xf,yf)) % 255.0;
        pngframe.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, 255]));
      }
    }
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, i-1 )).unwrap();
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, (((frames*2)-1)-(i-1)) )).unwrap();
    scale *= scale_mult
  }
  println!("\ngenclosures(): Successfully generated .png frames for .mp4");
  let _cmdffmpegc = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("ffmpeg -y -framerate 30 -i {}/{}_%d.png -c:v libx264 -pix_fmt yuv420p {}.mp4", pngdir, pngname, vidname)])
      .output()
      .expect("genclosures(): Failed to execute _cmdffmpegc")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("genclosures(): Failed to execute _cmdffmpegc")
  };
  println!("\ngenclosures(): Successfully executed _cmdffmpegc to generate {}.mp4", vidname);
  gencleanup(pngdir, pngname, frames);
  println!("\ngenclosures(): Successfully cleaned up extra frames in '{}'", pngdir);
}

fn genoverlay(
  pngname: &str, framesdir: &str, framename: &str, vidname: &str,
  frames: u32,
  mut ifactor: f64,
  ifactor_adj: f64,
  mut scale: f64,
  scale_mult: f64,
  f_r: impl Fn(f64, f64) -> f64,
  f_g: impl Fn(f64, f64) -> f64,
  f_b: impl Fn(f64, f64) -> f64,
  fr_theta: impl Fn(f64, f64) -> f64,
  fg_theta: impl Fn(f64, f64) -> f64,
  fb_theta: impl Fn(f64, f64) -> f64,
  edge_detect: bool
) {
  let pngimg = image::open(pngname.to_owned()+".png").expect(&format!("genoverlay(): could not find source image '{}.png' to use in overlay generation", pngname));
  let mut pngsrc: RgbaImage = pngimg.into_rgba8();
  if edge_detect {
    pngsrc = pngedges(pngsrc);
  }
  if !fs::metadata(framesdir).is_ok() {
    if let Err(err) = fs::create_dir_all(framesdir) {
      eprintln!("genoverlay(): Error creating directory '{}': {}", framesdir, err);
    } else {
      println!("\ngenoverlay(): Successfully created '{}'", framesdir);
    }
  } else {
    println!("\ngenoverlay(): '{}' already exists; cleaning directory contents...", framesdir);
    if let Err(err) = cleandir(framesdir) {
      eprintln!("genoverlay(): Error cleaning '{}': {}", framesdir, err);
    }
    println!("\ngenoverlay(): Successfully cleaned '{}'", framesdir);
  }
  let if_adj: f64 = ifactor_adj / (frames as f64);
  let mut pngframe = RgbaImage::new(pngsrc.width(), pngsrc.height());
  for i in 1..frames+1 {
    for x in 0..pngsrc.width() {
      for y in 0..pngsrc.height() {
        let pxlsrc: Rgba<u8> = image::Rgba(pngsrc.get_pixel(x, y).0);
        let xf = x as f64;
        let yf = y as f64;
        let r: f64 = (ifactor * (pxlsrc[0] as f64) + (1.0 - ifactor) * (fr_theta(xf,yf) * scale * f_r(xf,yf))) % 255.0;
        let g: f64 = (ifactor * (pxlsrc[1] as f64) + (1.0 - ifactor) * (fg_theta(xf,yf) * scale * f_g(xf,yf))) % 255.0;
        let b: f64 = (ifactor * (pxlsrc[2] as f64) + (1.0 - ifactor) * (fb_theta(xf,yf) * scale * f_b(xf,yf))) % 255.0;
        pngframe.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, 255]));
      }
    }
    pngframe.save(format!("{}/{}_{}.png", framesdir, framename, i-1 )).unwrap();
    pngframe.save(format!("{}/{}_{}.png", framesdir, framename, (((frames*2)-1)-(i-1)) )).unwrap();
    scale *= scale_mult;
    ifactor += if_adj;
    ifactor %= 1.0;
  }
  println!("\ngenoverlay(): Successfully generated .png frames for .mp4");
  let _cmdffmpego = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("ffmpeg -y -framerate 30 -i {}/{}_%d.png -c:v libx264 -pix_fmt yuv420p {}.mp4", framesdir, framename, vidname)])
      .output()
      .expect("genoverlay(): Failed to execute _cmdffmpego")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("genoverlay(): Failed to execute _cmdffmpego")
  };
  println!("\ngenoverlay(): Successfully executed _cmdffmpego to generate {}.mp4", vidname);
  gencleanup(framesdir, framename, frames);
  println!("\ngenoverlay(): Successfully cleaned up extra frames in '{}'", framesdir);
}

fn vfxoverlay(
  vidname: &str, framesdir: &str, outname: &str,
  mut ifactor: f64,
  ifactor_adj: f64,
  mut scale: f64,
  scale_adj: f64,
  f_r: impl Fn(f64, f64) -> f64,
  f_g: impl Fn(f64, f64) -> f64,
  f_b: impl Fn(f64, f64) -> f64,
  fr_theta: impl Fn(f64, f64) -> f64,
  fg_theta: impl Fn(f64, f64) -> f64,
  fb_theta: impl Fn(f64, f64) -> f64,
  edge_detect: bool
) {
  if !fs::metadata(vidname).is_ok() {
    eprintln!("vfxoverlay(): Could not locate '{}'", vidname);
  }
  if !fs::metadata(framesdir).is_ok() {
    if let Err(err) = fs::create_dir_all(framesdir) {
      eprintln!("vfxoverlay(): Error creating directory '{}': {}", framesdir, err);
    } else {
      println!("\ngenoverlay(): Successfully created '{}'", framesdir);
    }
  } else {
    println!("\nvfxoverlay(): '{}' already exists; cleaning directory contents...", framesdir);
    if let Err(err) = cleandir(framesdir) {
      eprintln!("vfxoverlay(): Error cleaning '{}': {}", framesdir, err);
    }
    println!("\nvfxoverlay(): Successfully cleaned '{}'", framesdir);
  }
  let parts: Vec<&str> = outname.split("/").collect();
  let shortoutname = parts[2];
  let _cmdteardown = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("ffmpeg -i {} -vf fps=30 {}/{}_%03d.png", vidname, framesdir, shortoutname)])
      .output()
      .expect("vfxoverlay(): Failed to execute _cmdteardown")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("vfxoverlay(): Failed to execute _cmdteardown")
  };
  println!("\nvfxoverlay(): Successfully executed _cmdteardown to teardown '{}' into frames", vidname);
  let framefiles: Vec<String> = match fs::read_dir(framesdir) {
    Ok(files) => {
      let fnames: Vec<String> = files.filter_map(|entry| {
        entry.ok().and_then(|e| e.file_name().into_string().ok())
      }).collect();
      fnames
    }
    Err(err) => {
      eprintln!("vfxoverlay(): Error reading output frames in '{}': {}", framesdir, err);
      Vec::new()
    }
  };
  let if_adj: f64 = ifactor_adj / (framefiles.len() as f64);
  for framename in framefiles {
    let fxname = framename.replace("_", "_fx_");
    let pngimg = image::open(&format!("{}/{}", framesdir, framename)).expect(&format!("vfxoverlay(): could not find source image '{}/{}' to use in overlay generation", framesdir, framename));
    let mut pngsrc: RgbaImage = pngimg.into_rgba8();
    if edge_detect {
      pngsrc = pngedges(pngsrc)
    }
    let mut pngframe: RgbaImage = RgbaImage::new(pngsrc.width(), pngsrc.height());
    for x in 0..pngsrc.width() {
      for y in 0..pngsrc.height() {
        let pxlsrc: Rgba<u8> = image::Rgba(pngsrc.get_pixel(x, y).0);
        let xf = x as f64;
        let yf = y as f64;
        let r: f64 = ((ifactor * (pxlsrc[0] as f64)) + ((1.0 - ifactor) * (fr_theta(xf,yf) * scale * f_r(xf,yf)))) % 255.0;
        let g: f64 = ((ifactor * (pxlsrc[1] as f64)) + ((1.0 - ifactor) * (fg_theta(xf,yf) * scale * f_g(xf,yf)))) % 255.0;
        let b: f64 = ((ifactor * (pxlsrc[2] as f64)) + ((1.0 - ifactor) * (fb_theta(xf,yf) * scale * f_b(xf,yf)))) % 255.0;
        pngframe.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, 255]));
      }
    }
    pngframe.save(format!("{}/{}", framesdir, fxname)).unwrap();
    match fs::remove_file(&format!("{}/{}", framesdir, framename)) { // cleanup unaffected frames to save space while building
      Ok(_) => {}
      Err(err) => {
        eprintln!("vfxoverlay(): Error removing frame '{}/{}': {}", framesdir, framename, err)
      }
    }
    scale *= scale_adj;
    ifactor += if_adj;
    ifactor %= 1.0;
  }
  let _cmdrebuild = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("ffmpeg -y -framerate 30 -i {}/{}_fx_%03d.png -c:v libx264 -pix_fmt yuv420p {}.mp4", framesdir, shortoutname, outname)])
      .output()
      .expect("vfxoverlay(): Failed to execute _cmdrebuild")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("vfxoverlay(): Failed to execute _cmdrebuild")
  };
  println!("\nvfxoverlay(): Successfully executed _cmdrebuild to generate {}.mp4", outname);
}

fn main() {
  /*genclosures(
    &String::from("src/png_out/test2.1"), // pngdir
    &String::from("test2.1"), // pngname
    &String::from("src/vid_out/test2.1"), // vidname
    1000, 1000, // width, height
    30, // frames
    1.42, 1.125, // scale, scale_mult
    |x, y| (x-y) as f64, // f_r()
    |x, y| (x*y) as f64, // f_g()
    |x, y| (x*x+y*y) as f64, // f_b()
    |x, y| ((x*y) * (x*x + y*y)) as f64, // fr_theta()
    |x, y| (((x*y).cos()) * (x*x - y*y)) as f64, // fg_theta()
    |x, y| ((x*y) * (x-y)) as f64, // fb_theta()
  );
  genoverlay(
    &String::from("src/png_in/test2.0_7"), // pngname
    &String::from("src/png_out/test3.1"), // framesdir
    &String::from("test3.1"), // framename
    &String::from("src/vid_out/test3.1"), // vidname
    30, // frames
    0.5, // ifactor
    0.0 // ifactor_adj
    1.42, // scale
    1.125, // scale_mult
    |x, y| (((x*y).sin()) * (x*x + y*y)) as f64, // f_r()
    |x, y| (((x*y).cos()) * (x*x - y*y)) as f64, // f_g()
    |x, y| (((x*y).tan()) * (x-y)) as f64, // f_b()
    |x, y| (x-y) as f64, // fr_theta()
    |x, y| (x*y) as f64, // fg_theta()
    |x, y| (x*x+y*y) as f64, // fb_theta()
    false // edge_detect
  );
  */vfxoverlay(
    &String::from("src/vid_in/stairs.mp4"), // vidname
    &String::from("src/png_out/stairs.2"), // framesdir
    &String::from("src/vid_out/stairs.2"), // outname
    0.925, // ifactor
    0.074, // ifactor_adj
    1.0, // scale,
    1.0, // scale_mult
    |x, y| ((x*x*y) + (x*y*y)) as f64, // f_r()
    |x, y| ((x + y) * (x - y)).abs() as f64, // f_g()
    |x, y| (x + y) as f64, // f_b()
    |x, y| ((x / (y + 1.0)) + (y / (x + 1.0))) as f64, // fr_theta()
    |x, y| ((x / (y + 1.0)) - (y / (x + 1.0))).abs() as f64, // fg_theta()
    |x, y| ((x / (y + 1.0)) * (y / (x + 1.0))) as f64, // fb_theta()
    false  // edge_detect
  );
}
