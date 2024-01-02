

use std::collections::HashMap;

use std::io::Write;
use crossterm::QueueableCommand;
use crossterm::style::Print;

mod config;
mod menus;


enum SelectMenu {
    KeySet,
    Temerament,
}

#[derive(Default)]
enum MenuState {
    Select(SelectMenu),
    #[default]
    Main,
    None,
}


pub struct UiThread {
    config: config::KeySet,
    shared_state: crate::state::KeyboardState,
    menu_state: MenuState,
}

impl std::ops::Drop for UiThread {
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl UiThread {

    pub fn new(shared_state: crate::state::KeyboardState) -> Self { Self {
        config: config::KeySet::default(),
        shared_state,
        menu_state: MenuState::default(),
    }}

    // run blocking
    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();
        'mainloop: loop {
            stdout.queue(
                crossterm::terminal::Clear ( crossterm::terminal::ClearType::All )
            )?;

            match self.menu_state {
                MenuState::Main => {
                    menus::main_menu(
                        &self.config,
                        self.shared_state.clone(),
                        &mut self.menu_state
                    )?;
                },
                MenuState::Select(SelectMenu::KeySet) => {
                    menus::keyset_menu(&mut self.config)?;
                    self.menu_state = MenuState::default();
                },
                MenuState::Select(SelectMenu::Temerament) => {
                    menus::temperament_menu(&mut self.config)?;
                    self.menu_state = MenuState::default();
                }
                MenuState::None => break 'mainloop,
            }
        }

        return Ok(());
    }

    pub fn init_drawing_area() -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();

        crossterm::terminal::enable_raw_mode()?;

        execute!(
            stdout,
            crossterm::event::PushKeyboardEnhancementFlags (
                crossterm::event::KeyboardEnhancementFlags::all()
            ),
            crossterm::cursor::Hide,
            crossterm::terminal::Clear ( crossterm::terminal::ClearType::All ),
        )?;

        return Ok(());
    }

    // restore the terminal to normal mod
    pub fn cleanup(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(std::io::stdout(), crossterm::event::PopKeyboardEnhancementFlags);
    }
}




