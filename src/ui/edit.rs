use crossterm::event::{KeyCode, KeyEvent};

use tui::{
    layout::Alignment,
    style::{Color, Style},
    text::{Span, Spans},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{
        Block, BorderType, Borders, Paragraph, Wrap,
    },
};

use crate::ui::{App, InputMode};
// use crate::explorer::{ Explorer, RipGrep };
use crate::explorer::Explorer;
use crate::explorer::rip_grep::RipGrep;


pub fn render_edit<'a>(rip_grep_command: &RipGrep, raw_output: &'a str, chunk: Rect, input_mode: InputMode) -> (Paragraph<'a>, Paragraph<'a>, Vec<Rect>) {
    let color = match input_mode {
        InputMode::Normal => Color::Gray,
        InputMode::Editing => Color::Red,
    };
    let edit_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [Constraint::Length(3),Constraint::Min(3)]
        )
        .split(chunk);

    let input = Paragraph::new(vec![
            Spans::from(vec![Span::raw(rip_grep_command.search_term_buffer.clone())]),
            Spans::from(vec![Span::raw("INPUT this should be mutable")]),
        ]) // app.input is a String
        .style(Style::default().fg(color))
        .block(Block::default().borders(Borders::ALL).title("Search term"));

    // let asd = rip_grep_command.raw_output().split("\n").collect::<Vec<String>>();
    /*
    let asd = rip_grep_command.raw_output().split("\n").collect::<Vec<&str>>();
    let asd: Vec<Span> = asd.into_iter().map(|line| Span::raw(line)).collect();
    */

    let asd: Vec<Span> = raw_output
        .split("\n")
        .map(|line| Span::raw(format!("{}\n", line)))
        .collect();

    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw(rip_grep_command.to_string())]),
        // Spans::from(vec![Span::raw(rip_grep_command.raw_output())]),
        // Spans::from(vec![Span::raw(rip_grep_command.raw_output())]),
        // Spans::from(rip_grep_command.raw_output().split("\n").collect::<Vec<&str>>().iter().map(|line| Span::raw(line))),
        // Spans::from(asd.iter().map(|line| Span::raw(line))),
        Spans::from(asd),
    ])
    .wrap(Wrap { trim: false })
    // .wrap(Wrap { trim: true })
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Rip Grep command to be executed:")
            .border_type(BorderType::Plain),
    );
    (input, home, edit_chunks)
}

pub fn action_edit(explorer: &mut Explorer, app: &mut App, key: KeyEvent) {
    match app.get_input_mode() {
        InputMode::Normal => {
            match key.code {
                KeyCode::Char('i') => { app.set_input_mode(InputMode::Editing); },
                _ => {}
            }
        },
        InputMode::Editing => {
            match key.code {
                KeyCode::Char(c) => { explorer.grep.search_term_buffer.push(c); },
                KeyCode::Backspace => { explorer.grep.search_term_buffer.pop(); }
                _ => {}
            }
        }
    }
}



