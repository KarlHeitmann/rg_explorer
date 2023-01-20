use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use std::io;

use tui::{
    backend::CrosstermBackend,
    Terminal
};
use clap::Parser;

mod rip_grep;
mod ui;
mod wrapper;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    search_term: String,
    folder: Option<String>,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // println!("name: {:?}\npath: {:?}", cli.search_term, cli.folder.as_deref());

    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let folder = if cli.folder.is_none() { String::from(".") } else { cli.folder.unwrap() };

    wrapper::explorer_wrapper(&mut terminal, cli.search_term, folder)?;

    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

