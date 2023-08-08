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

fn setup_logger(file_path: &str, log_level: &str) {
    // let level = match log_level.as_str() {
    let level = match log_level {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        "off" => log::LevelFilter::Off,
        _ => log::LevelFilter::Error,
    };

    // Build a stderr logger.
    let _stderr = log4rs::append::console::ConsoleAppender::builder().target(log4rs::append::console::Target::Stderr).build();

    // Logging to log file.
    let logfile = log4rs::append::file::FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new("{l} - {m}\n")))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = log4rs::config::Config::builder()
        .appender(log4rs::config::Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            log4rs::config::Root::builder()
                .appender("logfile")
                // .build(log::LevelFilter::Trace),
                .build(level),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    // let _handle = log4rs::init_config(config)?;
    let _handle = log4rs::init_config(config).unwrap();
}

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
    match std::env::var("LOG_LEVEL") {
        Ok(log_level) => { setup_logger(&std::env::var("LOG_FILE").unwrap_or(String::from("./log.txt")), &log_level) },
        _ => {}
    }

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

