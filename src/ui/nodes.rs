use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table,
    },
};

use crate::ui::NodeTabSelected;
use crate::nodes::RipGrep;
use crate::ui::App;

pub fn render_nodes<'a>(node_list_state: &ListState, rip_grep: &'a RipGrep, app: &App) -> (List<'a>, Table<'a>) {
    let folder_filter = app.folder_filter.clone();
    let selected_node_tab = &app.selected_node_tab;
    let (style_list, style_detail) = match selected_node_tab {
        NodeTabSelected::Detail  => { (Style::default().fg(Color::White), Style::default().fg(Color::Green)) },
        NodeTabSelected::FileList => { (Style::default().fg(Color::Green), Style::default().fg(Color::White)) },
    };
    let nodes_block:Block = Block::default()
        .borders(Borders::ALL)
        .style(style_list)
        .title(format!("Filter: '{folder_filter}'"))
        .border_type(BorderType::Plain);

    // let items: Vec<ListItem> = rip_grep.nodes.filtered_nodes(folder_filter)
    let items: Vec<ListItem> = rip_grep.nodes.filtered_nodes(folder_filter)
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

    let node_detail = rip_grep.node_detail(node_list_state.selected().expect("there is always a selected node"), app.offset_detail)
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Lines",
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


