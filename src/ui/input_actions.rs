use tui::widgets::ListState;
use crate::database::{operations::get_password_entires, structures::DatabaseFile};
use super::enums::MenuItem;


pub fn key_down(active_menu_item: MenuItem, password_entires_list_state: &mut ListState, db: &DatabaseFile){
    if active_menu_item == MenuItem::PasswordEntries{
        if let Some(selected) = password_entires_list_state.selected() {
            let amount_entries = get_password_entires(db).len();
            if selected >= amount_entries - 1 {
                password_entires_list_state.select(Some(0));
            } else {
                password_entires_list_state.select(Some(selected + 1));
            }
        }
    } else if active_menu_item == MenuItem::SelctedEntry {
       //Todo 
    }
}

pub fn key_up(active_menu_item: MenuItem, password_entires_list_state: &mut ListState, db: &DatabaseFile){
    if active_menu_item == MenuItem::PasswordEntries{
        if let Some(selected) = password_entires_list_state.selected() {
            let amount_entries = get_password_entires(db).len();
            if selected > 0 {
                password_entires_list_state.select(Some(selected - 1));
            } else {
                password_entires_list_state.select(Some(amount_entries - 1));
            }
        }              
    } else if active_menu_item == MenuItem::SelctedEntry {
        // TODO
    }
}
