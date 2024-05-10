use color_eyre::Result;
use input::Input;
mod app;
mod components;
mod errors;
mod input;
mod notify_mutex;
mod tabs;
pub mod tui;
use crate::app::App;

fn main() -> Result<()> {
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    let input = Input::new();
    App::new(input).run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
