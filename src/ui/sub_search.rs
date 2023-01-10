use crossterm::event::{KeyCode, KeyEvent};

use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph, ListState
    },
};

use crate::nodes::RipGrep;
use crate::ui::App;

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

pub fn action_nodes(_rip_grep: &mut RipGrep, app: &mut App, key: KeyEvent, _node_list_state: &mut ListState) {
    match key.code {
        KeyCode::Enter => {
            // Create Rip Grep instance
            // let rip_grep_child = RipGrep::new(app.subchild_search.clone());
        }
        KeyCode::Char(c) => {
            app.subchild_search.push(c)
        }
        _ => {}
    }
}

