use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph,
    },
};

pub fn render_sub_search<'a>(_rip_grep_command: String) -> Paragraph<'a> {
    let sub_search = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(">>>>>>>>>> TODO Sub Search <<<<<<<<<<<")]),
        Spans::from(vec![Span::raw("")]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Sub Search")
            .border_type(BorderType::Plain),
    );
    sub_search
}


