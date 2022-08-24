use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    widgets::{
        ListState,
    },
    Terminal,
};

use crate::database::{structures::DatabaseFile};

use super::{render::*, input_actions, menu_actions};
use super::structures::*;
use super::enums::*;

pub fn run_gui(db: DatabaseFile) -> Result<(), Box<dyn std::error::Error>> {
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
                    menu_actions::password_entires_menu(&mut password_entires_list_state, &db, rect, &chunks);
            },
                MenuItem::SelctedEntry => {
                    display_selected_entry(&db, &app, rect, &mut detail_list_state, password_entires_list_state.selected().unwrap(), &chunks)
                },
            }
            rect.render_widget(info, chunks[2]);
        })?;

        match rx.recv()? {
            Event::Tick => {},
            Event::Input(event) => {
                match app.input_mode {
                    InputMode::Navigation => match event.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Char('p') => {
                            active_menu_item = MenuItem::PasswordEntries
                        }
                        KeyCode::Char('h') => {
                            active_menu_item = MenuItem::Home
                        }
                        KeyCode::Char('s') => {
                            active_menu_item = MenuItem::SelctedEntry
                        }
                        KeyCode::Down => {
                            input_actions::key_down(active_menu_item, &mut password_entires_list_state, &db);
                        }
                        KeyCode::Up => {
                            input_actions::key_up(active_menu_item, &mut password_entires_list_state, &db);
                        }
                        _ => {} 
                    }
                    InputMode::Editing => match event.code{
                        
                        _ => {}
                    },
                    _ => {},
                }
            }
        }
    }
    Ok(())
}

