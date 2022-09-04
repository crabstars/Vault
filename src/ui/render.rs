use std::usize;
use std::io::Stdout;

use chrono::Local;
use anyhow::Error;
use tui::{
    layout::{Alignment, Constraint, Layout, Direction, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Frame,
    backend::CrosstermBackend,
};

use crate::database::{structures::{PasswordEntry, EntryType, DatabaseFile}, operations::get_password_entires};
use super::enums::MenuItem;
use super::structures::App;

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
                .unwrap(),
        )
        .unwrap_or(&PasswordEntry{id: String::from("1"), title: String::from("Empty"), value: String::from("Empty"), name: String::from("Empty"), 
            url: String::from("Empty"),comment: String::from("Empty"), entry_type: EntryType::ClassicPassword, last_modified: Local::now()})
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
        Cell::from(Span::raw("*****")),
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
        MenuItem::SelctedEntry => vec!["home", "password-entries", "edit-value", "ESC-quit-edit", "copy-value", "show-secret", "quit"],
        MenuItem::PasswordEntries => vec!["home", "password-entries", "select-entry", "add-entry", "quit"],
        _ => vec!["home", "password-entries", "quit"],

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

pub fn display_selected_entry(db: &DatabaseFile, app: &App, rect: &mut Frame<CrosstermBackend<Stdout>>,
                              detail_list_state: &mut ListState, selected_entry: usize, chunks: &[Rect], show_value: &bool) -> usize{
    let entry_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),)
                        .split(chunks[1]);
                    
    let right_chunks = Layout::default().direction(Direction::Vertical).constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),)
                        .split(entry_chunks[1]);

    let (left, bottom_right, top_right, attribute_count) = render_selected_entry(selected_entry, detail_list_state, app, db, show_value);
    rect.render_stateful_widget(left, entry_chunks[0], detail_list_state);
    rect.render_widget(bottom_right, right_chunks[1]);
    rect.render_widget(top_right, right_chunks[0]);
    attribute_count
}

fn render_selected_entry<'a>(index: usize, detail_list_state: &ListState, app: &'a App, db: &DatabaseFile, show_value: &bool) -> (List<'a>, Paragraph<'a>, Paragraph<'a>, usize){
    let props = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Changeable Properties")
        .border_type(BorderType::Plain);

    let default = PasswordEntry{id: String::from("1"), title: String::from("Empty"), value: String::from("Empty"), name: String::from("Empty"), url: String::from("Empty"),
                        comment: String::from("Empty"), entry_type: EntryType::ClassicPassword, last_modified: Local::now()};
    let selected_entry = get_password_entires(db).get(index).unwrap_or(&default).clone();

    let names: Vec<String> = ["Title".into(), "Name".into(), "Value".into(), "Url".into(), "Comment".into()].to_vec();

    let items: Vec<_> = names
        .iter()
        .map(|name| {
            ListItem::new(Spans::from(vec![Span::styled(
                name.clone(),
                Style::default(),
            )]))
        })
        .collect();

    let list = List::new(items).block(props).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD),
    );


    let mut value = match detail_list_state.selected().unwrap() {
        0 => selected_entry.title,
        1 => selected_entry.name,
        2 => selected_entry.value,
        3 => selected_entry.url,
        4 => selected_entry.comment,
        _ => "Error".to_owned()
    };
    if !*show_value && detail_list_state.selected().unwrap() == 2{
        value = (0..value.len()).map(|_| "*").collect::<String>();
    }

    let detail = Paragraph::new(vec![
        Spans::from(vec![Span::raw(value)]),
    ]);


    let mut text = vec![];


    if !app.input.clone().is_empty(){

        let mut left_side = app.input[..app.input_index-1].to_string(); 
        // TODO extract this function (because im using it 3 times)
        if !*show_value && detail_list_state.selected().unwrap() == 2{
            left_side = (0..left_side.len()).map(|_| "*").collect::<String>(); 
        }
        if app.input.len() > app.input_index-1{
            let mut middle = app.input.chars().nth(app.input_index-1).unwrap_or_default().to_string();
            let mut right_side = app.input[app.input_index..].to_string();
            if !*show_value && detail_list_state.selected().unwrap() == 2{
                left_side = (0..left_side.len()).map(|_| "*").collect::<String>(); 
                middle = (0..middle.len()).map(|_| "*").collect::<String>();
                right_side = (0..right_side.len()).map(|_| "*").collect::<String>();
            }
              text = vec![
                Spans::from(vec![
                    Span::styled(left_side, Style::default().fg(Color::Blue)), //before index
                    Span::styled(middle, 
                                 Style::default().add_modifier(Modifier::BOLD)), // at index
                    Span::styled(right_side, Style::default().fg(Color::Blue))
                ])
            ];

        } else{
            text = vec![
                Spans::from(vec![
                    Span::styled(left_side, Style::default().fg(Color::Blue)), //before index
                    Span::styled("_",Style::default().add_modifier(Modifier::SLOW_BLINK)), // at index
                ])
            ];
        }
    }


    let input_field = Paragraph::new(text)
        .block(Block::default().title("Input-Field").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);


    (list, detail, input_field, names.len()) 
}
