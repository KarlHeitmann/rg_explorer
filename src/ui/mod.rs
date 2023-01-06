use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph, Tabs,
    },
};

pub mod home;
pub mod edit;
pub mod nodes;

use crate::MenuItem;

#[derive(Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    input_mode: InputMode,
}

impl Default for App {
    fn default() -> App {
        App {
            input_mode: InputMode::Normal,
        }
    }
}

impl App {
    pub fn get_input_mode(&self) -> InputMode {
        self.input_mode
    }
    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.input_mode = input_mode;
    }
}

pub fn get_layout_chunks(size: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size)
}

// pub fn draw_menu_tabs(menu_titles: &Vec<&str>, active_menu_item: MenuItem) -> Tabs {
pub fn draw_menu_tabs<'a>(menu_titles: &'a Vec<&'a str>, active_menu_item: MenuItem) -> Tabs<'a> {
    let menu = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(menu)
        .select(active_menu_item.into())
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}

pub fn draw_status_bar<'layout>(input_mode: InputMode) -> Paragraph<'layout> {
    let (title, color) = match input_mode {
        InputMode::Normal =>  ("NORMAL MODE", Color::LightCyan),
        InputMode::Editing => ("-Insert mode-", Color::Red),
    };
    Paragraph::new(title)
        .style(Style::default().fg(color))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Status")
                .border_type(BorderType::Plain),
        )
}


