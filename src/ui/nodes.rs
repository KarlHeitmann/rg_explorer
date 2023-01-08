use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table,
    },
};

use crate::nodes::RipGrep;

// pub fn render_nodes<'a>(node_list_state: &ListState, nodes: &'a Nodes) -> (List<'a>, Table<'a>) {
pub fn render_nodes<'a>(node_list_state: &ListState, rip_grep: &'a RipGrep) -> (List<'a>, Table<'a>) {
    let nodes_block:Block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("RG Explorer")
        .border_type(BorderType::Plain);

    let items: Vec<_> = rip_grep.nodes
        .0.iter()
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

    // let node_detail = rip_grep.nodes_detail(node_list_state.selected().expect("there is always a selected node")
    /*
    let node_detail = rip_grep.nodes
        // .0.get
        .0.get(
            node_list_state
                .selected()
                .expect("there is always a selected node"),
        )
        .expect("exists")
        .detail()
        */
    let node_detail = rip_grep.node_detail(node_list_state.selected().expect("there is always a selected node"))
        .header(Row::new(vec![
            Cell::from(Span::styled(
                "Lines",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Cell::from(Span::styled(
                "Category",
                Style::default().add_modifier(Modifier::BOLD),
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ]);

    (list, node_detail)
}


