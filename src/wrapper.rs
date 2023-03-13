use crossterm::event::{self, Event, KeyCode};
use std::io;
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Terminal,
};
use crate::ui;

use crate::ui::nodes::action_nodes;
use crate::ui::edit::action_edit;
use crate::ui::sub_search::{render_sub_search, action_sub_search};
use crate::ui::edit::render_edit;
use crate::ui::nodes::render_nodes;
use crate::ui::home::render_home;
use crate::explorer::Explorer;

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
    SubSearch
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Nodes => 1,
            MenuItem::Edit => 2,
            MenuItem::SubSearch => 3,
        }
    }
}

fn selection_menu_handler(key_code: KeyCode) -> Option <MenuItem> {
    match key_code {
        KeyCode::Char('h') => Some(MenuItem::Home),
        KeyCode::Char('n') => Some(MenuItem::Nodes),
        KeyCode::Char('e') => Some(MenuItem::Edit),
        KeyCode::Char('s') => Some(MenuItem::SubSearch),
        _ => None,
    }
}
use std::io::Stdout;

 // pub fn explorer(terminal: Terminal<B>) {
pub fn explorer_wrapper(terminal: &mut Terminal<CrosstermBackend<Stdout>>, search_term: String, folders: String, word: Option<String>, ignorecase: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut explorer = Explorer::new(search_term, folders, word.clone(), ignorecase.clone()); // TODO Create default
    let mut app = ui::App::default();
    let mut active_menu_item = MenuItem::Home;
    let mut node_list_state = ListState::default(); // TARGET
    node_list_state.select(Some(0));
    let menu_titles = vec!["Home", "Nodes", "Edit", "SubSearch", "Delete", "Quit"];

    loop {
        terminal.draw(|rect| {
            let explorer = &mut explorer;
            let chunks = ui::get_layout_chunks(rect.size());

            let status_bar = ui::draw_status_bar(&app, &explorer);

            let tabs = ui::draw_menu_tabs(&menu_titles, active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(explorer.to_string()), chunks[1]),
                MenuItem::Nodes => {
                    explorer.run_wrapper();
                    if explorer.nodes.len() == 0{
                        // TODO: Put a message saying no results
                    } else {
                        let nodes_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                            )
                            .split(chunks[1]);
                        let (left, right) = render_nodes(&node_list_state, &explorer, &app);
                        rect.render_stateful_widget(left, nodes_chunks[0], &mut node_list_state);
                        rect.render_widget(right, nodes_chunks[1]);
                    }
                },
                MenuItem::Edit => {
                    // let raw_output = explorer.grep.raw_output().as_str();
                    let raw_output = explorer.grep.raw_output();
                    let (first, second, edit_chunks) = render_edit(&explorer.grep, &raw_output, chunks[1], app.get_input_mode());
                    rect.render_widget(first, edit_chunks[0]);
                    rect.render_widget(second, edit_chunks[1]);
                    match app.get_input_mode() {
                        ui::InputMode::Editing => { 
                            rect.set_cursor(
                                edit_chunks[0].x + explorer.grep.search_term_buffer.len() as u16 + 1,
                                edit_chunks[0].y + 1,
                            )

                        },
                        _ => {},
                    }
                },
                MenuItem::SubSearch => rect.render_widget(render_sub_search(explorer.to_string(), app.subchild_search.to_string()), chunks[1]),
            }
            rect.render_widget(status_bar, chunks[2]);
        })?;

        // let terminal = &mut terminal;
        if let Event::Key(key) = event::read()? {
            match app.get_input_mode() {
                ui::InputMode::Normal => {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                    let menu_item = selection_menu_handler(key.code);
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
                    action_edit(&mut explorer, &mut app, key);
                },
                MenuItem::Nodes => {
                    action_nodes(terminal, &mut explorer, &mut app, key, &mut node_list_state, )?;
                },
                MenuItem::SubSearch => {
                    action_sub_search(terminal, explorer.get_file_name_matches(), &mut app, key, word.clone(), ignorecase.clone())?;
                },
                _ => {
                    match key.code {
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

