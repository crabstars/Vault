use anyhow::Error;
use chrono::{Local};
use tui::{
    layout::{Alignment, Constraint, Layout, Direction, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

use crate::database::{structures::{PasswordEntry, EntryType, DatabaseFile}, operations::get_password_entires};
use super::enums::MenuItem;

pub fn render_home<'a>() -> Paragraph<'a> {
    let home = Paragraph::new(vec![
        Spans::from(vec![Span::styled(
            "Welcome to Vault",
            Style::default().fg(Color::LightBlue),
        )]),
        Spans::from(vec![Span::raw("Your personal terminal password manager")]),
        Spans::from(vec![Span::raw("")]),
        Spans::from(vec![Span::raw("Press 'p' to access password entries")]),
        Spans::from(vec![Span::raw("Press 'a' to add new entries")]),
        Spans::from(vec![Span::raw("Press 's' to select an entry")]),
        Spans::from(vec![Span::raw("Press 'd' to delete an entry")]),


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

pub fn render_password_entires<'a>(password_entries_list_state: &ListState, db: &DatabaseFile) -> (List<'a>, Table<'a>){
    let entires = Block::default()
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::White))
    .title("Passwords")
    .border_type(BorderType::Plain);

    let entry_list = get_password_entires(db);
    let items: Vec<_> = entry_list
        .iter()
        .map(|entry| {
            ListItem::new(Spans::from(vec![Span::styled(
                entry.name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let selected_entry = entry_list
        .get(
            password_entries_list_state
                .selected()
                .expect("there is always a selected entry"),
        )
        .expect("exists")
        .clone();

    let list = List::new(items).block(entires).highlight_style(
        Style::default()
            .bg(Color::Red)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );
    let entry_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_entry.title)),
        Cell::from(Span::raw(selected_entry.name)),
        Cell::from(Span::raw(selected_entry.value)),
        Cell::from(Span::raw(selected_entry.comment)),
        Cell::from(Span::raw(selected_entry.last_modified.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "Title",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Name",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Value",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Comment",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "Last Modified at",
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
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ]);

    (list, entry_detail)
}

pub fn render_info<'a>() -> Paragraph<'a>{
    return Paragraph::new("FOSS password manager and more")
    .style(Style::default().fg(Color::LightCyan))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Info")
            .border_type(BorderType::Plain),
    );
}

pub fn render_tabs<'a>( active_menu_item: MenuItem) -> Tabs<'a>{
    return Tabs::new(get_menu_for_mode(&active_menu_item))
    .select(active_menu_item.into())
    .block(Block::default().title("Menu").borders(Borders::ALL))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Yellow))
    .divider(Span::raw("|"));
}


fn get_menu_for_mode<'a>(active_menu_item: &MenuItem) -> Vec<Spans<'a>> {
    let men = match active_menu_item {
        MenuItem::SelctedEntry => vec!["Home", "Password-Entries", "Edit-Value", "ESC-Quit-Edit", "Quit"],
        _ => vec!["Home", "Password-Entries", "Quit"],

    };

    return men
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
}

pub fn render_chunks(size: Rect) -> Vec<Rect>{
    return Layout::default()
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
                .split(size);
}