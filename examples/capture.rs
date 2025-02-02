use gphoto2::{Context, Result};
use std::path::Path;

fn main() -> Result<()> {
  env_logger::init();

  let camera = Context::new()?.autodetect_camera()?;

  let file = camera.capture_image()?;
  println!("Captured image {}", file.name());

  camera.fs().download_to(&file.folder(), &file.name(), Path::new(&file.name().to_string()))?;
  println!("Downloaded image {}", file.name());

  Ok(())
}
