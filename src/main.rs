use std::io;

use crate::app::App;

mod app;

fn main() -> Result<(), io::Error> {
    App::new().run()?;
    Ok(())
}