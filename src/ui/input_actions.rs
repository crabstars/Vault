use super::enums::MenuItem;
use super::enums::*;
use super::structures::*;
use crate::database::operations::Database;
use crate::database::structures::DatabaseFile;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use tui::widgets::ListState;

pub fn key_down(
    active_menu_item: MenuItem,
    password_entires_list_state: &mut ListState,
    detail_list_state: &mut ListState,
    db: &DatabaseFile,
    attribute_count: &usize,
) {
    if active_menu_item == MenuItem::PasswordEntries {
        let amount_entries = db.entries.len();
        if amount_entries == 0 {
            return;
        }
        if let Some(selected) = password_entires_list_state.selected() {
            if selected >= amount_entries - 1 {
                password_entires_list_state.select(Some(0));
            } else {
                password_entires_list_state.select(Some(selected + 1));
            }
        }
    } else if active_menu_item == MenuItem::SelctedEntry {
        if let Some(selected) = detail_list_state.selected() {
            if selected >= attribute_count - 1 {
                detail_list_state.select(Some(0));
            } else {
                detail_list_state.select(Some(selected + 1));
            }
        }
    }
}

pub fn key_up(
    active_menu_item: MenuItem,
    password_entires_list_state: &mut ListState,
    detail_list_state: &mut ListState,
    db: &DatabaseFile,
    attribute_count: &usize,
) {
    if active_menu_item == MenuItem::PasswordEntries {
        let amount_entries = db.entries.len();
        if amount_entries == 0 {
            return;
        }
        if let Some(selected) = password_entires_list_state.selected() {
            if selected > 0 {
                password_entires_list_state.select(Some(selected - 1));
            } else {
                password_entires_list_state.select(Some(amount_entries - 1));
            }
        }
    } else if active_menu_item == MenuItem::SelctedEntry {
        if let Some(selected) = detail_list_state.selected() {
            if selected > 0 {
                detail_list_state.select(Some(selected - 1));
            } else {
                detail_list_state.select(Some(attribute_count - 1));
            }
        }
    }
}

pub fn key_enter(
    app: &mut App,
    password_entires_list_state: &mut ListState,
    detail_list_state: &mut ListState,
    db: &mut DatabaseFile,
) {
    app.message.push(app.input.drain(..).collect());
    let index_entries = password_entires_list_state.selected().unwrap();
    let index_detail = detail_list_state.selected().unwrap();

    db.update_entry(
        index_detail,
        db.entries[index_entries].id.clone(),
        app.message.clone(),
    );
    app.input_mode = InputMode::Navigation;
    app.input_index = 0;
    app.input = String::new();
}

pub fn key_code_c(
    active_menu_item: MenuItem,
    password_entires_list_state: &ListState,
    detail_list_state: &ListState,
    db: &DatabaseFile,
) {
    if active_menu_item == MenuItem::SelctedEntry {
        let index_entries = password_entires_list_state.selected().unwrap();
        let index_detail = detail_list_state.selected().unwrap();

        let value = match index_detail {
            0 => db.entries[index_entries].title.clone(),
            1 => db.entries[index_entries].name.clone(),
            2 => db.entries[index_entries].value.clone(),
            3 => db.entries[index_entries].url.clone(),
            4 => db.entries[index_entries].comment.clone(),
            _ => "".to_owned(),
        };
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(value).unwrap();
    }
}
