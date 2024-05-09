use color_eyre::Result;
mod app;
mod errors;
mod tabs;
mod components;
mod input;
mod notify_mutex;
pub mod tui;
use crate::app::App;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::new().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
