use std::usize;
use std::io::Stdout;

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

pub fn display_selected_entry(db: &DatabaseFile, app: &App, rect: &mut Frame<CrosstermBackend<Stdout>>, detail_list_state: &mut ListState, selected_entry: usize, chunks: &Vec<Rect>){

    let pets_chunks = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(chunks[1]);
                    
    let right_chunks = Layout::default().direction(Direction::Vertical).constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(pets_chunks[1]);

    let (left, bottom_right, top_right) = render_selected_entry(selected_entry, detail_list_state, app, db);
    rect.render_stateful_widget(left, pets_chunks[0], detail_list_state);
    rect.render_widget(bottom_right, right_chunks[1]);
    rect.render_widget(top_right, right_chunks[0]); 
}

fn render_selected_entry<'a>(index: usize, detail_list_state: &ListState, app: &'a App, db: &DatabaseFile) -> (List<'a>, Paragraph<'a>, Paragraph<'a>){
    let props = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .title("Properties")
        .border_type(BorderType::Plain);

    let selected_entry = get_password_entires(db).get(index).expect("Error while getting element").clone();

    let names: Vec<String> = ["Title".into(), "Name".into(), "Value".into(), "Url".into(), "Comment".into(), "Last modified".into()].to_vec();

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


    let value = match detail_list_state.selected().unwrap() {
        0 => selected_entry.title,
        1 => selected_entry.name,
        2 => selected_entry.value,
        3 => selected_entry.url,
        4 => selected_entry.comment,
        5 => selected_entry.last_modified.to_string(),
        _ => "Error".to_owned()
    };

    let detail = Paragraph::new(vec![
        Spans::from(vec![Span::raw(value)]),
    
    ]);


    let mut text = vec![];

    // TODO improve code because DRY
    if app.input.clone().len() != 0 && app.input.len() > app.input_index-1{
          text = vec![
            Spans::from(vec![
                Span::styled(&app.input[..app.input_index-1], Style::default().fg(Color::Blue)), //before index
                Span::styled(app.input.chars().nth(app.input_index-1).unwrap_or_default().to_string(),Style::default().add_modifier(Modifier::BOLD)), // at index
                Span::styled(&app.input[app.input_index..], Style::default().fg(Color::Red))
            ])
        ];

    } else if app.input.clone().len() != 0{
        text = vec![
            Spans::from(vec![
                Span::styled(&app.input[..app.input_index-1], Style::default().fg(Color::Blue)), //before index
                Span::styled("_",Style::default().add_modifier(Modifier::SLOW_BLINK)), // at index
            ])
        ];
    }

    text.push(Spans::from(vec![
        Span::styled(app.input_index.to_string(), Style::default().fg(Color::Green)), //before index
    ]));

   
   

    let input_field = Paragraph::new(text)
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center);


    (list, detail, input_field) 
}
