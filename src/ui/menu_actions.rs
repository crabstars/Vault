use std::io::Stdout;
use tui::{layout::{Layout, Direction, Constraint, Rect}, widgets::ListState, Frame, backend::CrosstermBackend};
use crate::database::structures::DatabaseFile;
use super::render::render_password_entires;


pub fn password_entires_menu(password_entires_list_state: &mut ListState, db: &DatabaseFile, 
    rect: &mut Frame<CrosstermBackend<Stdout>>, chunks: &Vec<Rect>){

    let password_entry_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
        )
        .split(chunks[1]);
    let (left, right) = render_password_entires(&password_entires_list_state, &db);
    rect.render_stateful_widget(left, password_entry_chunks[0], password_entires_list_state);
    rect.render_widget(right, password_entry_chunks[1]);
}