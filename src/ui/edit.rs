use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph,
    },
};

pub fn render_edit<'a>(rip_grep_command: String) -> Paragraph<'a> {
    /*
    let input = Paragraph::new(app.input.as_ref()) // app.input is a String
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    */
    let input = Paragraph::new(vec![
            Spans::from(vec![Span::raw(rip_grep_command)]),
            Spans::from(vec![Span::raw("INPUT this should be mutable")]),
        ]) // app.input is a String
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    let _home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Edit")]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Home")
            .border_type(BorderType::Plain),
    );
    input
}



