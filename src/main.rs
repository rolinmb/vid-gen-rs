use std::fs;
use std::path::Path;
use std::process::Command;
use image::{Rgba, RgbaImage};
/*fn fneval(stringexpr: &str, x: u32, y: u32) -> f64 {
  //println!("{}", stringexpr);
  let mut cmdexprstr = stringexpr.replace(char::is_whitespace, "");
  cmdexprstr = cmdexprstr.replace("(", "[");
  cmdexprstr = cmdexprstr.replace(")", "]");
  cmdexprstr = "[".to_owned()+&cmdexprstr;
  cmdexprstr = cmdexprstr+"]";
  //println!("{}", cmdexprstr);
  let cmdeval = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", "src\\main.exe", &cmdexprstr, &x.to_string(), &y.to_string()])
      .output()
      .expect("fneval(): Failed to execute eval command")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello from fneval() in main.rs")
      .output()
      .expect("fneval(): Failed to execute eval command")
  };
  let evaloutput = String::from_utf8_lossy(&cmdeval.stdout);
  //println!("{}", evaloutput);
  match evaloutput.trim().parse::<f64>() {
    Ok(parsednum) => parsednum,
    Err(err) => {
      eprintln!("\nfneval(): Error parsing {} to f64: {}", evaloutput, err);
      0.0
    }
  }
}*/

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
  f_r: impl Fn(f32, f32) -> f64,
  f_g: impl Fn(f32, f32) -> f64,
  f_b: impl Fn(f32, f32) -> f64,
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
        let xfloat = x as f32;
        let yfloat = y as f32;
        pngframe.put_pixel(
          x, y,
          Rgba(
            [
              ((scale * f_r(xfloat,yfloat)) % 256.0) as u8,
              ((scale * f_g(xfloat,yfloat)) % 256.0) as u8,
              ((scale * f_b(xfloat,yfloat)) % 256.0) as u8,
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
      .args(["/C", &format!("ffmpeg -y -framerate {} -i {}/{}_%d.png -c:v libx264 -pix_fmt yuv420p {}.mp4", frames, pngdir, pngname, vidname)])
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
}/*

fn genstrings(
  pngdir: &str, pngname: &str, vidname: &str,
  width: u32, height: u32, frames: u32,
  str_r: &str, str_g: &str, str_b: &str,
) {
  if !fs::metadata(pngdir).is_ok() {
    if let Err(err) = fs::create_dir_all(pngdir) {
      eprintln!("genstrings(): Error creating directory '{}': {}", pngdir, err);
    } else {
      println!("\ngenstrings(): Successfully created '{}'", pngdir);
    }
  } else {
    println!("genstrings(): '{}' already exists; cleaning directory contents...", pngdir);
    if let Err(err) = cleandir(pngdir) {
      eprintln!("genstrings(): Error cleaning '{}': {}", pngdir, err);
    }
    println!("\ngenstrings(): Successfully cleaned '{}'", pngdir);
  }
  let mut pngframe = RgbaImage::new(width, height);
  for i in 1..frames+1 {
    for x in 0..width {
      for y in 0..height {
        pngframe.put_pixel(
          x, y,
          Rgba([fneval(str_r,x,y) as u8, fneval(str_g,x,y) as u8, fneval(str_b,x,y) as u8, 255]),
        );
      }
    }
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, i - 1 )).unwrap();
    pngframe.save(format!("{}/{}_{}.png", pngdir, pngname, (((frames*2)-1)-(i-1)) )).unwrap();
  }
  println!("\ngenstrings(): Successfully generated .png frames for .mp4");
  let _cmdffmpeg = if cfg!(target_os = "windows") {
    Command::new("cmd")
      .args(["/C", &format!("ffmpeg -y -framerate {} -i {}/{}_%d.png -c:v libx264 -pix_fmt yuv420p {}.mp4", frames, pngdir, pngname, vidname)])
      .output()
      .expect("genstrings(): Failed to execute ffmpeg command")
  } else {
    Command::new("sh")
      .arg("-c")
      .arg("echo hello")
      .output()
      .expect("genstrings(): Failed to execute ffmpeg command")
  };
  println!("\ngenstrings(): Successfully executed ffmpeg command to generate {}", vidname);
  gencleanup(pngdir, pngname, frames);
  println!("\ngenstrings(): Successfully cleaned up extra frames in '{}'", pngdir);
}

*/fn main() {
  genclosures(
    &String::from("src/png_out/test0"),
    &String::from("test0"),
    &String::from("src/vid_out/test0"),
    1000, 1000, 10, 1.0, 1.1,
    |x, y| (x-y) as f64,
    |x, y| (x*y) as f64,
    |x, y| (x*x+y*y) as f64,
  );
  //println!("{}", fneval(&String::from("(((pow(y , 2 + x)) / (1 + x))*sin( y ))"), 2, 10));
  /*genstrings(
    &String::from("src/png_out/test1"),
    &String::from("test1"),
    &String::from("src/vid_out/test1"),
    1000, 1000, 10,
    &String::from("sin(x)+cos(y)+tan(x+y)"),
    &String::from("x*y"),
    &String::from("x*x+y*y"),
  );*/
}
