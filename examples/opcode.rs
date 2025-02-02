//! GPhoto cannot implement all features of all cameras,
//! but you can send custom PTP opcodes to the camera.
//!
//! This example starts and ends live view mode on the camera for 10 s,
//! this example only works on Nikon cameras.

use gphoto2::widget::TextWidget;
use gphoto2::{Context, Result};
use std::{thread, time::Duration};

fn main() -> Result<()> {
  env_logger::init();

  let camera = Context::new()?.autodetect_camera()?;

  let opcode = camera.config_key::<TextWidget>("opcode")?;

  println!("Starting live view");
  opcode.set_value("0x9201")?;
  camera.set_config(&opcode)?;

  thread::sleep(Duration::from_secs(10));

  println!("Ending live view");
  opcode.set_value("0x9202")?;
  camera.set_config(&opcode)?;

  Ok(())
}
