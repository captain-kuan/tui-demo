use crate::{tabs, tui};
use color_eyre::{
    eyre::{bail, Ok, WrapErr},
    Result,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

pub struct Environment {}

pub struct App {}

impl App {
    pub fn new() -> Result<Self> {
        Ok(App {})
    }
    pub fn run() -> Result<()> {
        Ok(())
    }
}
