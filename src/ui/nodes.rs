use crossterm::event::{KeyCode, KeyEvent};
use std::io::Stdout;

use tui::{
    layout::Constraint,
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table,
    },
    Terminal,
};

use crate::ui::NodeTabSelected;
use crate::explorer::Explorer;
use crate::ui::{App, InputMode, FilterMode};
use crate::wrapper::explorer_wrapper;

pub fn render_nodes<'a>(node_list_state: &ListState, explorer: &'a Explorer, app: &App) -> (List<'a>, Table<'a>) {
    let selected_node_tab = &app.selected_node_tab;
    let (style_list, style_detail) = match selected_node_tab {
        NodeTabSelected::Detail  => { (Style::default().fg(Color::White), Style::default().fg(Color::Green)) },
        NodeTabSelected::FileList => {
            match explorer.filter_mode {
                FilterMode::Contain => {
                    (Style::default().fg(Color::Green), Style::default().fg(Color::White))
                }
                FilterMode::Omit => {
                    (Style::default().fg(Color::Red), Style::default().fg(Color::White))
                }

            }
        },
    };
    let nodes_block:Block = Block::default()
        .borders(Borders::ALL)
        .style(style_list)
        .title(format!("Filter: '{}'", explorer.show_folder_filter()))
        .border_type(BorderType::Plain);

    let items: Vec<ListItem> = explorer.filtered_nodes()
        .into_iter()
        .map(|node| {
            ListItem::new(Spans::from(vec![Span::styled(
                node.summary(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(nodes_block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let (file_name, node_detail) = explorer.node_detail(node_list_state.selected().expect("there is always a selected node"), app.offset_detail);
    let node_detail = node_detail
        .header(Row::new(vec![
            Cell::from(Span::styled(
                format!(" {}", file_name),
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(style_detail)
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(100),
        ]);

    (list, node_detail)
}

pub fn action_nodes(terminal: &mut Terminal<CrosstermBackend<Stdout>>, title: String, explorer: &mut Explorer, app: &mut App, key: KeyEvent, node_list_state: &mut ListState) -> Result<(), Box<dyn std::error::Error>> {
    match app.get_input_mode() {
        InputMode::Normal => {
            match key.code {
                KeyCode::Char('i') => app.set_input_mode(InputMode::Editing),
                KeyCode::Char('c') => { 
                    explorer.filter_mode = FilterMode::Contain;
                    // TODO: run again filter to refresh nodes with filter mode
                },
                KeyCode::Char('o') => {
                    explorer.filter_mode = FilterMode::Omit
                    // TODO: run again filter to refresh nodes with filter mode
                },
                _ => {}
            }
        }
        InputMode::Editing => {
            explorer.update_folder_filter(key.code);
        }
    }
    match key.code {
        // KeyCode::Left => { explorer.decrease_context(); },
        KeyCode::Left => { explorer.update_context(node_list_state.selected().unwrap(), -1); },
        KeyCode::Right => { explorer.update_context(node_list_state.selected().unwrap(), 1); },
        KeyCode::Down => {
            match app.selected_node_tab {
                NodeTabSelected::FileList => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = explorer.filtered_nodes().len(); // TODO: Consider borrow instead of clone
                        if amount_nodes > 0 {
                            if selected >= amount_nodes - 1 {
                                node_list_state.select(Some(0));
                            } else {
                                node_list_state.select(Some(selected + 1));
                            }
                        }
                    }
                    app.offset_detail = 0;
                }
                NodeTabSelected::Detail => {
                    if let Some(selected) = node_list_state.selected() {
                        if app.offset_detail < explorer.nodes.node_matches_count(selected) { app.offset_detail += 1; }
                    }
                }
            }
        }
        KeyCode::Up => {
            match app.selected_node_tab {
                NodeTabSelected::FileList => {
                    if let Some(selected) = node_list_state.selected() {
                        let amount_nodes = explorer.filtered_nodes().len(); // TODO: Consider borrow instead of clone
                        if amount_nodes > 0 {
                            if selected > 0 {
                                node_list_state.select(Some(selected - 1));
                            } else {
                                node_list_state.select(Some(amount_nodes - 1));
                            }
                        }
                    }
                    app.offset_detail = 0;
                }
                NodeTabSelected::Detail => {
                    if app.offset_detail > 0 { app.offset_detail -= 1; }
                }
            }
        }
        KeyCode::Tab => {app.selected_node_tab = if app.selected_node_tab == NodeTabSelected::FileList { NodeTabSelected::Detail } else { NodeTabSelected::FileList} }
        KeyCode::Enter => {
            let selected = node_list_state.selected().unwrap();
            let node = explorer.get_node(selected).unwrap();
            let complete_file_name = node.file_name();
            let (folder, file_name) = complete_file_name.rsplit_once("/").unwrap();
            let (first_char, file_name) = file_name.split_at(1);
            match first_char {
                "_" => {
                    if folder.contains("app/views") {
                        let (name, _) = file_name.split_once(".").unwrap();
                        explorer_wrapper(terminal, &format!("{} {} ", title, complete_file_name), format!("render.*{}", name), String::from("app/views"), false, false)?;
                    }
                },
                _ => {}
            }
        }
        KeyCode::Backspace => {
        }
        _ => {}
    }
    Ok(())
}

