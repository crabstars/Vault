use tui::{
    layout::{Alignment},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

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
        _ => vec!["Home", "Password-Entries", "Add", "Select", "Delete", "Quit"],

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
