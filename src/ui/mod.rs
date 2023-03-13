use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph, Tabs,
    },
};

use crate::explorer::Explorer;

pub mod home;
pub mod edit;
pub mod nodes;
pub mod sub_search;

use crate::wrapper::MenuItem;

#[derive(Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
pub enum NodeTabSelected {
    FileList,
    Detail,
}

pub enum FilterMode {
    Contain,
    Omit,
}

pub struct App {
    selected_node_tab: NodeTabSelected,
    pub offset_detail: usize,
    pub subchild_search: String,
    input_mode: InputMode,
}

impl Default for App {
    fn default() -> App {
        App {
            offset_detail: 0,
            subchild_search: String::from(""),
            // selected_node_tab: String::from(""),
            selected_node_tab: NodeTabSelected::FileList,
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
pub fn draw_menu_tabs<'a>(menu_titles: &'a Vec<&'a str>, active_menu_item: MenuItem, title: String) -> Tabs<'a> {
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
        .block(Block::default().title(title).borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}

pub fn draw_status_bar<'layout>(app: &App, explorer: &Explorer) -> Paragraph<'layout> {
    let (title, color) = match app.get_input_mode() {
        InputMode::Normal => {
            match explorer.filter_mode {
                FilterMode::Contain => ("NORMAL MODE +++FILTER MODE CONTAIN+++", Color::LightCyan),
                FilterMode::Omit => ("NORMAL MODE ---filter mode omit---", Color::LightCyan),
            }
        },
        InputMode::Editing => {
            match explorer.filter_mode {
                FilterMode::Contain => ("-Insert mode- +++FILTER MODE CONTAIN+++", Color::Red),
                FilterMode::Omit => ("-Insert mode- ---filter mode omit---", Color::Red),
            }
        }
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


