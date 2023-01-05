use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use thiserror::Error;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

mod nodes;
// mod io::run_command;
mod io_rg;
use crate::io_rg::RipGrep;
use crate::nodes::Nodes;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Nodes,
    Edit,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Nodes => 1,
            MenuItem::Edit => 2,
        }
    }
}

fn run(results: Vec<&str>) -> Nodes {
    Nodes::new(results)
}
/*
fn run(results: Vec<&str>) {
    let parsed_result = nodes::RgExplorer::new(results);
    println!("{}", parsed_result);
}
*/

fn get_layout_chunk(size: Rect) -> Vec<Rect> {
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

fn draw_menu_tabs(menu_titles: Vec<&str>, active_menu_item: MenuItem) -> Tabs {
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

fn draw_copyright<'layout>() -> Paragraph<'layout> {
    Paragraph::new("pet-CLI 2020 - all rights reserved")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let main_nodes = read_db().expect("can't fetch nodes for rg explorer");
    let search_term = "fn";
    // let search_term = "a";
    // let search_term = ";";
    // let search_term = "an";
    let rip_grep = RipGrep::new(search_term);
    // let rip_grep = RipGrep { search_term: "fn" } ;
    // let rip_grep = RipGrep { search_term: String::from("fn")} ;
    let results = rip_grep.run_command();
    let main_nodes = run(results.split("\n").collect::<Vec<&str>>());

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    // let tick_rate = Duration::from_millis(2000);
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate { // This if block makes everything "change", cos it's actually updating. Comment the block to not get so confused with the TUI
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // let mut active_menu_item = MenuItem::Home;
    let mut active_menu_item = MenuItem::Edit;
    let mut pet_list_state = ListState::default();
    pet_list_state.select(Some(0));

    loop {
        let menu_titles = vec!["Home", "Nodes", "Edit", "Add", "Delete", "Quit"];
        terminal.draw(|rect| {
            let chunks = get_layout_chunk(rect.size());

            let copyright = draw_copyright();

            let tabs = draw_menu_tabs(menu_titles, active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(rip_grep.to_string()), chunks[1]),
                // MenuItem::Home => rect.render_widget(render_home(rip_grep), chunks[1]),
                MenuItem::Nodes => {
                    let pets_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    let (left, right) = render_pets(&pet_list_state, &main_nodes);
                    rect.render_stateful_widget(left, pets_chunks[0], &mut pet_list_state);
                    rect.render_widget(right, pets_chunks[1]);
                },
                MenuItem::Edit => rect.render_widget(render_edit(rip_grep.to_string()), chunks[1]),
            }
            rect.render_widget(copyright, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('n') => active_menu_item = MenuItem::Nodes,
                KeyCode::Char('e') => active_menu_item = MenuItem::Edit,
                KeyCode::Down => {
                    if let Some(selected) = pet_list_state.selected() {
                        let amount_pets = main_nodes.len();
                        if selected >= amount_pets - 1 {
                            pet_list_state.select(Some(0));
                        } else {
                            pet_list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = pet_list_state.selected() {
                        let amount_pets = main_nodes.len();
                        if selected > 0 {
                            pet_list_state.select(Some(selected - 1));
                        } else {
                            pet_list_state.select(Some(amount_pets - 1));
                        }
                    }
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn render_home<'a>(rip_grep_command: String) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw(rip_grep_command)]),
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
    home
}

fn render_edit<'a>(rip_grep_command: String) -> Paragraph<'a> {
    let home = Paragraph::new(vec![
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
    home
}

fn render_pets<'a>(pet_list_state: &ListState, all_pets: &'a Nodes) -> (List<'a>, Table<'a>) {
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

