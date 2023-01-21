use crossterm::event::{KeyCode, KeyEvent};

use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table,
    },
};

use crate::ui::NodeTabSelected;
use crate::explorer::Explorer;
use crate::ui::{App, InputMode, FilterMode};

pub fn render_nodes<'a>(node_list_state: &ListState, explorer: &'a Explorer, app: &App) -> (List<'a>, Table<'a>) {
    let folder_filter = app.folder_filter.clone();
    let selected_node_tab = &app.selected_node_tab;
    let (style_list, style_detail) = match selected_node_tab {
        NodeTabSelected::Detail  => { (Style::default().fg(Color::White), Style::default().fg(Color::Green)) },
        NodeTabSelected::FileList => {
            match app.filter_mode {
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
        .title(format!("Filter: '{folder_filter}'"))
        .border_type(BorderType::Plain);

    // let items: Vec<ListItem> = explorer.nodes.filtered_nodes(folder_filter)
    let items: Vec<ListItem> = explorer.nodes.filtered_nodes(&folder_filter, &app.filter_mode)
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

    let detail_title = explorer.get_node(node_list_state.selected().expect("there is always a selected node"));

    let node_detail = explorer.node_detail(node_list_state.selected().expect("there is always a selected node"), app.offset_detail)
        .header(Row::new(vec![
            Cell::from(Span::styled(
                format!(" {}", detail_title.file_name()),
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

pub fn action_nodes(explorer: &mut Explorer, app: &mut App, key: KeyEvent, node_list_state: &mut ListState) {
    match app.get_input_mode() {
        InputMode::Normal => {
            match key.code {
                KeyCode::Char('i') => app.set_input_mode(InputMode::Editing),
                KeyCode::Char('c') => app.filter_mode = FilterMode::Contain,
                KeyCode::Char('o') => app.filter_mode = FilterMode::Omit,
                _ => {}
            }
        }
        InputMode::Editing => {
            match key.code {
                KeyCode::Char(c) => { app.folder_filter.push(c); },
                KeyCode::Backspace => { app.folder_filter.pop(); },
                _ => {}
            }
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
                        let amount_nodes = explorer.nodes.filtered_nodes(&app.folder_filter, &app.filter_mode).len(); // TODO: Consider borrow instead of clone
                        if selected >= amount_nodes - 1 {
                            node_list_state.select(Some(0));
                        } else {
                            node_list_state.select(Some(selected + 1));
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
                        let amount_nodes = explorer.nodes.filtered_nodes(&app.folder_filter, &app.filter_mode).len(); // TODO: Consider borrow instead of clone
                        if selected > 0 {
                            node_list_state.select(Some(selected - 1));
                        } else {
                            node_list_state.select(Some(amount_nodes - 1));
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
        KeyCode::Enter => {}
        KeyCode::Backspace => {
        }
        _ => {}
    }
}

