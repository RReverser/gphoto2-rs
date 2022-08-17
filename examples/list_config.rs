use gphoto2::{Context, Result};

fn main() -> Result<()> {
  let context = Context::new()?;
  let camera = context.autodetect_camera()?;

  println!("{:?}", camera.config()?);

  Ok(())
}
