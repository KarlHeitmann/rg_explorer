use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
// use crossterm::terminal::disable_raw_mode;

use std::io;

use tui::{
    backend::CrosstermBackend,
    Terminal
};

mod nodes;
mod ui;
mod wrapper;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_term = "fn";

    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    wrapper::rip_grep_wrapper(&mut terminal, search_term.to_string(), String::from("."));

    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

