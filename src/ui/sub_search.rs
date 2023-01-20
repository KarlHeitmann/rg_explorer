use crossterm::event::{KeyCode, KeyEvent};
use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph
    },
    Terminal,
};

use crate::ui::{App, InputMode};
use crate::wrapper::explorer_wrapper;

pub fn render_sub_search<'a>(_rip_grep_command: String, subchild_search: String) -> Paragraph<'a> {
    let sub_search = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(subchild_search)]),
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

pub fn action_sub_search(terminal: &mut Terminal<CrosstermBackend<Stdout>>, file_name_matches: String, app: &mut App, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
    match app.get_input_mode() {
        InputMode::Normal => {
            match key.code {
                KeyCode::Char('i') => { app.set_input_mode(InputMode::Editing); },
                _ => {}
            }
        },
        InputMode::Editing => {
            match key.code {
                KeyCode::Enter => { explorer_wrapper(terminal, app.subchild_search.clone(), file_name_matches)?; app.set_input_mode(InputMode::Normal); }
                KeyCode::Char(c) => { app.subchild_search.push(c); }
                KeyCode::Backspace => { app.subchild_search.pop(); },
                _ => {}
            }
        }
    }; Ok(())
}

