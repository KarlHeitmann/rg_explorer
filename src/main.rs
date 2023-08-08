use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};


use std::io;

use tui::{
    backend::CrosstermBackend,
    Terminal
};
use clap::Parser;

mod explorer;
mod ui;
mod wrapper;


/*
#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'f')]
    eff: bool,

    #[arg(short = 'p', value_name = "PEAR")]
    pea: Option<String>,

    #[arg(last = true)]
    slop: Vec<String>,
}
*/


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    search_term: String,
    #[arg(short = 'w')]
    word: bool,
    #[arg(short = 'i')]
    ignorecase: bool,
    folder: Option<String>,
}

fn reset_terminal() -> std::result::Result<(), Box<dyn std::error::Error>> {
    terminal::disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // println!("name: {:?}\npath: {:?}", cli.search_term, cli.folder.as_deref());

    enable_raw_mode().expect("can run in raw mode");
    // crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;
    crossterm::execute!(io::stderr(), EnterAlternateScreen)?;


    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));



    let folder = if cli.folder.is_none() { String::from(".") } else { cli.folder.unwrap() };

    let title = String::from("  ");

    wrapper::explorer_wrapper(&mut terminal, &title, cli.search_term, folder, cli.word, cli.ignorecase)?;

    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

