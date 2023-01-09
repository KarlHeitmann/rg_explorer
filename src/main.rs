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
use crate::ui::sub_search::render_sub_search;
use crate::ui::home::render_home;
use crate::ui::edit::render_edit;
use crate::ui::nodes::render_nodes;
use crate::ui::NodeTabSelected;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_term = "fn";
    let mut rip_grep = nodes::RipGrep::new(search_term.to_string()); // TODO Create default
    let mut app = ui::App::default();

    enable_raw_mode().expect("can run in raw mode");

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;
    let mut node_list_state = ListState::default();
    node_list_state.select(Some(0));
    let menu_titles = vec!["Home", "Nodes", "Edit", "SubSearch", "Delete", "Quit"];

    loop {
        terminal.draw(|rect| {
            let rip_grep = &mut rip_grep;
            let chunks = ui::get_layout_chunks(rect.size());

            let status_bar = ui::draw_status_bar(app.get_input_mode());

            let tabs = ui::draw_menu_tabs(&menu_titles, active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(rip_grep.to_string()), chunks[1]),
                MenuItem::Nodes => {
                    rip_grep.run_wrapper();
                    if rip_grep.nodes.len() == 0{
                        // TODO: Put a message saying no results
                    } else {
                        let nodes_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints(
                                [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                            )
                            .split(chunks[1]);
                        let (left, right) = render_nodes(&node_list_state, &rip_grep, &app);
                        rect.render_stateful_widget(left, nodes_chunks[0], &mut node_list_state);
                        rect.render_widget(right, nodes_chunks[1]);
                    }
                },
                MenuItem::Edit => {
                    let (first, second, edit_chunks) = render_edit(&rip_grep, chunks[1], app.get_input_mode());
                    rect.render_widget(first, edit_chunks[0]);
                    rect.render_widget(second, edit_chunks[1]);
                    match app.get_input_mode() {
                        ui::InputMode::Editing => { 
                            rect.set_cursor(
                                edit_chunks[0].x + rip_grep.search_term_buffer.len() as u16 + 1,
                                edit_chunks[0].y + 1,
                            )

                        },
                        _ => {},
                    }
                },
                MenuItem::SubSearch => rect.render_widget(render_sub_search(rip_grep.to_string()), chunks[1]),
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
                        KeyCode::Char('s') => Some(MenuItem::SubSearch),
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
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
                                KeyCode::Char('i') => { app.set_input_mode(ui::InputMode::Editing); },
                                _ => {}
                            }
                        },
                        ui::InputMode::Editing => {
                            match key.code {
                                KeyCode::Char(c) => { rip_grep.search_term_buffer.push(c); },
                                KeyCode::Backspace => { rip_grep.search_term_buffer.pop(); }
                                _ => {}
                            }
                        }
                    }
                },
                MenuItem::Nodes => {
                    match app.get_input_mode() {
                        ui::InputMode::Normal => {
                            match key.code {
                                KeyCode::Char('i') => app.set_input_mode(ui::InputMode::Editing),
                                _ => {}
                            }
                        }
                        ui::InputMode::Editing => {
                            match key.code {
                                KeyCode::Char(c) => { app.folder_filter.push(c); },
                                KeyCode::Backspace => { app.folder_filter.pop(); },
                                _ => {}
                            }
                        }
                    }
                    match key.code {
                        KeyCode::Left => { rip_grep.decrease_context(); },
                        KeyCode::Right => { rip_grep.increase_context(); },
                        KeyCode::Down => {
                            match app.selected_node_tab {
                                NodeTabSelected::FileList => {
                                    if let Some(selected) = node_list_state.selected() {
                                        let amount_nodes = rip_grep.nodes.filtered_nodes(app.folder_filter.clone()).len();
                                        if selected >= amount_nodes - 1 {
                                            node_list_state.select(Some(0));
                                        } else {
                                            node_list_state.select(Some(selected + 1));
                                        }
                                    }
                                }
                                NodeTabSelected::Detail => {
                                    if let Some(selected) = node_list_state.selected() {
                                        if app.offset_detail < rip_grep.nodes.node_matches_count(selected) { app.offset_detail += 1; }
                                    }
                                }
                            }
                        }
                        KeyCode::Up => {
                            match app.selected_node_tab {
                                NodeTabSelected::FileList => {
                                    if let Some(selected) = node_list_state.selected() {
                                        let amount_nodes = rip_grep.nodes.filtered_nodes(app.folder_filter.clone()).len();
                                        if selected > 0 {
                                            node_list_state.select(Some(selected - 1));
                                        } else {
                                            node_list_state.select(Some(amount_nodes - 1));
                                        }
                                    }
                                }
                                NodeTabSelected::Detail => {
                                    if app.offset_detail > 0 { app.offset_detail -= 1; }
                                }
                            }
                        }
                        KeyCode::Tab => {app.selected_node_tab = if app.selected_node_tab == NodeTabSelected::FileList { NodeTabSelected::Detail } else { NodeTabSelected::FileList} }
                        KeyCode::Enter => {}
                        KeyCode::Backspace => {
                        }
                        _ => {}
                    }
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

