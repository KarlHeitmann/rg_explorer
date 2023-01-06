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

// use crate::io_rg::RipGrep;
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
    let mut rip_grep = nodes::RipGrep::new(search_term.to_string()); // TODO Create default
    let mut app = ui::App::default();
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

            let status_bar = ui::draw_status_bar(app.get_input_mode());

            let tabs = ui::draw_menu_tabs(&menu_titles, active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(rip_grep.to_string()), chunks[1]),
                // MenuItem::Home => rect.render_widget(render_home(rip_grep), chunks[1]),
                MenuItem::Nodes => {
                    let results = rip_grep.run_command();
                    let main_nodes = run(results.split("\n").collect::<Vec<&str>>());
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
            rect.render_widget(status_bar, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.get_input_mode() {
                ui::InputMode::Normal => {
                    let menu_item: Option<MenuItem> = match key.code {
                        KeyCode::Char('h') => Some(MenuItem::Home),
                        KeyCode::Char('n') => Some(MenuItem::Nodes),
                        KeyCode::Char('e') => Some(MenuItem::Edit),
                        _ => None,
                    };
                    match menu_item {
                        Some(menu_item) => {
                            active_menu_item = menu_item;
                            continue;
                        },
                        _ => {},
                    }
                },
                ui::InputMode::Editing => {
                    match key.code {
                        KeyCode::Esc|KeyCode::F(2) => { app.set_input_mode(ui::InputMode::Normal); continue; } // Gets traped in vim
                        _ => {}
                    }
                },
            }
            match active_menu_item {
                MenuItem::Edit => {
                    match app.get_input_mode() {
                        ui::InputMode::Normal => {
                            match key.code {
                                KeyCode::Char('i') => {
                                    app.set_input_mode(ui::InputMode::Editing);
                                },
                                _ => {}
                            }
                        },
                        ui::InputMode::Editing => {
                            match key.code {
                                KeyCode::Char(c) => {
                                    rip_grep.search_term.push(c)
                                },
                                KeyCode::Backspace => {
                                    rip_grep.search_term.pop();
                                }
                                _ => {}
                            }
                        }
                    }
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

