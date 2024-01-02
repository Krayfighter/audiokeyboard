

use std::collections::HashMap;

use std::io::Write;
use crossterm::QueueableCommand;
use crossterm::style::Print;

mod config;
mod menus;



#[derive(Default, Debug)]
enum MenuState {
    #[default]
    Main,
    None,
    Keyset,
    Temperament,
    Voice,
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
    pub fn run(&mut self, voice_mutex: crate::state::SyncFunctionPtr) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();
        'mainloop: loop {
            stdout.queue(
                crossterm::terminal::Clear ( crossterm::terminal::ClearType::All )
            )?;

            match self.menu_state {
                MenuState::Main => {
                    self.menu_state = menus::main_menu(
                        &self.config,
                        self.shared_state.clone(),
                    )?;
                },
                MenuState::Keyset => {
                    menus::keyset_menu(&mut self.config)?;
                    self.menu_state = MenuState::default();
                },
                MenuState::Temperament => {
                    menus::temperament_menu(&mut self.config)?;
                    self.menu_state = MenuState::default();
                },
                MenuState::Voice => {
                    menus::voice_menu(voice_mutex.clone())?;
                    self.menu_state = MenuState::default();
                },
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




