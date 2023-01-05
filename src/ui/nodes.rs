use tui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Row, Table,
    },
};

use crate::Nodes;

pub fn render_nodes<'a>(pet_list_state: &ListState, all_pets: &'a Nodes) -> (List<'a>, Table<'a>) {
    let pets = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("RG Explorer")
        .border_type(BorderType::Plain);

    let pet_list = all_pets;
    let items: Vec<_> = pet_list
        .0.iter()
        .map(|node| {
            ListItem::new(Spans::from(vec![Span::styled(
                node.summary(), // TODO: replace by something like "pet," or pet.name.clone(), memorare: pet.name was a String!!!
                Style::default(),
            )]))
        })
        .collect();

    let selected_pet = pet_list
        .0.get(
            pet_list_state
                .selected()
                .expect("there is always a selected pet"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(pets).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );

    let pet_detail = selected_pet.detail()
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

    (list, pet_detail)
}


