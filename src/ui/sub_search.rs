use crossterm::event::{KeyCode, KeyEvent};
use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph, ListState
    },
    Terminal,
};

use crate::nodes::RipGrep;
use crate::ui::App;
use crate::wrapper::rip_grep_wrapper;

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

// pub fn action_sub_search(terminal: &mut Terminal<CrosstermBackend<Stdout>>, rip_grep: &mut RipGrep, app: &mut App, key: KeyEvent) {
// pub fn action_sub_search(terminal: mut Terminal<CrosstermBackend<Stdout>>, rip_grep: &mut RipGrep, app: &mut App, key: KeyEvent) {
// pub fn action_sub_search(terminal: &mut Terminal<CrosstermBackend<Stdout>>, rip_grep: &mut RipGrep, app: &mut App, key: KeyEvent) {
// pub fn action_sub_search(terminal: &mut Terminal<CrosstermBackend<Stdout>>, rip_grep: RipGrep, app: &mut App, key: KeyEvent) {
pub fn action_sub_search(terminal: &mut Terminal<CrosstermBackend<Stdout>>, file_name_matches: String, app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Enter => {
            // Create Rip Grep instance
            // let rip_grep_child = RipGrep::new(app.subchild_search.clone());
            // rip_grep_wrapper(&mut terminal, "fun".to_string(), rip_grep.get_file_name_matches());
            // let s = rip_grep.get_file_name_matches();
            rip_grep_wrapper(terminal, "fun".to_string(), file_name_matches);
        }
        KeyCode::Char(c) => {
            app.subchild_search.push(c)
        }
        _ => {}
    }
}

