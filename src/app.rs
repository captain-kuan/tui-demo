use crate::{input::Input, tabs, tui};
use color_eyre::{
    eyre::{bail, Ok, WrapErr},
    Result,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

pub struct Environment {}

pub struct App {
    input: Input,
}

impl App {
    pub fn new(input: Input) -> Result<Self> {
        Ok(App { input })
    }
    pub fn run() -> Result<()> {
        Ok(())
    }
}
