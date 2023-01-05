use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Terminal,
};

mod nodes;
mod ui;

mod io_rg;
use crate::io_rg::RipGrep;
use crate::nodes::Nodes;
use crate::ui::home::render_home;
use crate::ui::edit::render_edit;
use crate::ui::nodes::render_nodes;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

#[derive(Copy, Clone, Debug)]
pub enum MenuItem {
    Home,
    Nodes,
    Edit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Nodes => 1,
            MenuItem::Edit => 2,
        }
    }
}

fn run(results: Vec<&str>) -> Nodes {
    Nodes::new(results)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_term = "fn";
    // let search_term = "a";
    // let search_term = ";";
    // let search_term = "an";
    let rip_grep = RipGrep::new(search_term.to_string()); // TODO Create default
    // let rip_grep = RipGrep { search_term: "fn" } ;
    // let rip_grep = RipGrep { search_term: String::from("fn")} ;
    let results = rip_grep.run_command();
    let main_nodes = run(results.split("\n").collect::<Vec<&str>>());

    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;
    let mut pet_list_state = ListState::default();
    pet_list_state.select(Some(0));
    let menu_titles = vec!["Home", "Nodes", "Edit", "Add", "Delete", "Quit"];

    loop {
        terminal.draw(|rect| {
            let chunks = ui::get_layout_chunks(rect.size());

            let copyright = ui::draw_copyright();

            let tabs = ui::draw_menu_tabs(&menu_titles, active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(rip_grep.to_string()), chunks[1]),
                // MenuItem::Home => rect.render_widget(render_home(rip_grep), chunks[1]),
                MenuItem::Nodes => {
                    let pets_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_nodes(&pet_list_state, &main_nodes);
                    rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
                    rect.render_widget(right, pets_chunks[1]);
                },
                MenuItem::Edit => rect.render_widget(render_edit(rip_grep.to_string()), chunks[1]),
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('n') => active_menu_item = MenuItem::Nodes,
                KeyCode::Char('e') => active_menu_item = MenuItem::Edit,
                _ => {}
            }
            match active_menu_item {
                MenuItem::Edit => { // TODO: THREAD is messing things up. Erase it. https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/
                },
                _ => {
                    match key.code {
                        KeyCode::Down => {
                            if let Some(selected) = pet_list_state.selected() {
                                let amount_pets = main_nodes.len();
                                if selected >= amount_pets - 1 {
                                    pet_list_state.select(Some(0));
                                } else {
                                    pet_list_state.select(Some(selected + 1));
                                }
                            }
                        }
                        KeyCode::Up => {
                            if let Some(selected) = pet_list_state.selected() {
                                let amount_pets = main_nodes.len();
                                if selected > 0 {
                                    pet_list_state.select(Some(selected - 1));
                                } else {
                                    pet_list_state.select(Some(amount_pets - 1));
                                }
                            }
                        }
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

