use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use anyhow::Ok;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    widgets::ListState,
    Terminal,
};

use crate::database::{structures::DatabaseFile, operations::Database};

use super::{render::*, input_actions, menu_actions};
use super::structures::*;
use super::enums::*;

pub fn run_gui(db: &mut DatabaseFile) -> Result<(), anyhow::Error> {
    enable_raw_mode().expect("can run in raw mode");
    let mut app = App::default();

    let (tx, rx) = mpsc::channel();
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

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                    last_tick = Instant::now();
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut active_menu_item = MenuItem::Home;

    let mut password_entires_list_state = ListState::default();
    password_entires_list_state.select(Some(0));

    let mut detail_list_state = ListState::default();
    detail_list_state.select(Some(0));
    let mut attribute_count = 0;
    let mut show_value = false;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = render_chunks(size);

            let info = render_info();
            let tabs = render_tabs(active_menu_item);

            rect.render_widget(tabs, chunks[0]);
            match active_menu_item {
                MenuItem::Home => rect.render_widget(render_home(), chunks[1]),
                MenuItem::PasswordEntries => {
                    menu_actions::password_entires_menu(&mut password_entires_list_state, db, rect, &chunks);
            },
                MenuItem::SelctedEntry => {
                    attribute_count = display_selected_entry(db, &app, rect, &mut detail_list_state,
                                                             password_entires_list_state.selected().unwrap(), &chunks, &show_value)
                },
            }
            rect.render_widget(info, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Tick => {},
            Event::Input(event) => {
                match app.input_mode {
                    InputMode::Navigation => match event.code {
                        KeyCode::Char('a') => {
                            if active_menu_item == MenuItem::PasswordEntries{
                                db.add_empty_entry();
                                active_menu_item = MenuItem::SelctedEntry;
                                password_entires_list_state.select(Some(db.entries.len()-1));
                            }
                        }
                        KeyCode::Char('c') => {
                            input_actions::key_code_c(active_menu_item, &password_entires_list_state,
                                                      &detail_list_state, db);
                        }
                        KeyCode::Char('e') => {
                            if active_menu_item == MenuItem::SelctedEntry{
                                app.input_mode = InputMode::Editing;

                                let index_entries = password_entires_list_state.selected().unwrap();
                                let index_detail = detail_list_state.selected().unwrap();
                                app.input = db.get_value_from_selected_detail(index_detail, db.entries[index_entries].id.clone());
                                app.input_index = app.input.len()+1;
                            }
                        }
                        KeyCode::Char('h') => {
                            active_menu_item = MenuItem::Home;
                            show_value = false;
                        }
                        KeyCode::Char('p') => {
                            active_menu_item = MenuItem::PasswordEntries;
                            show_value = false;
                        }
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('r') =>{
                            let password_len = db.entries.len();
                            if active_menu_item == MenuItem::PasswordEntries && password_len != 0 && password_len > password_entires_list_state.selected().unwrap_or(0){
                                db.remove_entry_by_id(db.entries[password_entires_list_state.selected().unwrap()].id.clone());
                            }
                        }
                        KeyCode::Char('s') => {
                            if active_menu_item == MenuItem::PasswordEntries{
                                let password_len = db.entries.len();
                                if password_len != 0 && password_len > password_entires_list_state.selected().unwrap_or(0){
                                    active_menu_item = MenuItem::SelctedEntry
                                }
                            } else if active_menu_item == MenuItem::SelctedEntry{
                                show_value = !show_value;
                            }
                        }
                        KeyCode::Down => {
                            input_actions::key_down(active_menu_item, &mut password_entires_list_state,
                                                    &mut detail_list_state, db, &attribute_count);
                        }
                        KeyCode::Up => {
                            input_actions::key_up(active_menu_item, &mut password_entires_list_state,
                                                  &mut detail_list_state, db, &attribute_count);
                        }
                        _ => {} 
                    }
                    InputMode::Editing => match event.code{
                        KeyCode::Enter => {
                            input_actions::key_enter(&mut app, &mut password_entires_list_state, &mut detail_list_state, db)
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Navigation;
                            app.input_index = 0;
                            app.input = String::new();
                        }
                        KeyCode::Char(c) => {
                            if app.input_index == 0{
                                app.input.insert(app.input_index, c);
                                app.input_index += 1;
                            } else{
                                app.input.insert(app.input_index-1, c);
                            }
                            
                            app.input_index += 1;
                            
                        }
                        KeyCode::Backspace => {
                            if !app.input.is_empty() && app.input_index > 1{
                                app.input.remove(app.input_index-2);
                                app.input_index -= 1;                                
                            }
                        }
                        KeyCode::Right =>{
                            if app.input_index < app.input.len()+1{
                                app.input_index += 1
                            }
                        }
                        KeyCode::Left =>{
                            if app.input_index > 1{
                                app.input_index -= 1
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

