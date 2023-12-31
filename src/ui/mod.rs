

use std::collections::HashMap;

use std::io::Write;
use crossterm::QueueableCommand;

mod input;

const DEFAULT_KEYS: &[char] = &['1', 'q', '2', 'w', 'e', '4', 'r', '5', 't', 'y', '7', 'u', '8', 'i', '9', 'o', 'p', '-', '[', '=', ']'];

const BOTTOM_KEY: u8 = 48;
const TOP_KEY: u8 = 68;

fn generate_default_keys() -> HashMap<char, u8> {
    let range = BOTTOM_KEY..=TOP_KEY;
    return DEFAULT_KEYS
            .iter()
            .cloned()
            .zip(range.into_iter())
            .collect();
}


struct Config {
    // the number of equal tones in the current scale
    num_tet: f32,
    keys: HashMap<char, u8>,
    keys_src: &'static [char],
}

impl Default for Config {
    fn default() -> Self {
        return Self {
            num_tet: 12., // 12TET
            keys: generate_default_keys(),
            keys_src: DEFAULT_KEYS,
        }
    }
}

impl Config {

    // does the equal temperament calculation described in the follwing
    // https://en.wikipedia.org/wiki/12_equal_temperament#Mathematical_properties
    #[inline]
    fn key_freq(&self, key_number: u8) -> f32 {
        const MIDDLE_A: f32 = 440.;
        const MIDDLE_A_KEY_NUM: i8 = 49;

        return MIDDLE_A * 2.0f32.powf (
            (key_number as i8 - MIDDLE_A_KEY_NUM) as f32 /
            self.num_tet
        );
    }

    pub fn get_freq(&self, key: char) -> Option<f32> {
        return match self.keys.get(&key) {
            Some(num) => Some ( Self::key_freq(&self, *num) ),
            None => None,
        }
    }

    pub fn get_key_index(&self, key: &char) -> anyhow::Result<usize> {
        return self.keys_src.iter()
            .position(|src_key| src_key == key)
            .ok_or(anyhow!("unable to find key position"));
    }
}


#[derive(Clone, Copy)]
struct PrintKeyCommand {
    bg_color: crossterm::style::Color,
    height: u16,
    width: u16,
    pad_top: u16,
    x_pos: u16,
}

impl Default for PrintKeyCommand {
    fn default() -> Self {
        return Self {
            bg_color: crossterm::style::Color::Magenta,
            height: 10,
            width: 5,
            pad_top: 6,
            x_pos: 17,
        }
    }
}

impl PrintKeyCommand {
    pub fn queue(
        &self,
        stdout: &mut std::io::Stdout,
    ) -> anyhow::Result<()> {
        // create a string of spaces `width` long
        let line_string = (0..self.width)
            .into_iter()
            .map(|_| " ")
            .fold(String::default(), |acc, str| acc + str);

        stdout.queue(crossterm::style::SetBackgroundColor ( self.bg_color ))?;

        for h in 0..self.height {
            stdout.queue(crossterm::cursor::MoveTo(self.x_pos, self.pad_top + h))?;
            stdout.queue(crossterm::style::Print(line_string.clone()))?;
        }

        stdout.queue(crossterm::style::ResetColor)?;
        return Ok(());
    }
}



pub struct UiThread {
    config: Config,
    shared_state: crate::state::KeyboardState,
}

impl std::ops::Drop for UiThread {
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl UiThread {

    pub fn new(shared_state: crate::state::KeyboardState) -> Self { Self {
        config: Config::default(),
        shared_state,
    }}

    // run blocking
    pub fn run(&mut self) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();
        'mainloop: loop {
            if let Err(_) = crossterm::event::poll( std::time::Duration::from_millis(50) ) { continue 'mainloop; }

            stdout.queue( crossterm::terminal::Clear ( crossterm::terminal::ClearType::All ) )?;

            // process events
            if let crossterm::event::Event::Key(keyevent) = crossterm::event::read()? {
                use crossterm::event::KeyCode as KC;
                use crossterm::event::KeyEventKind as KEK;
                match keyevent.code {
                    KC::Esc => break 'mainloop,
                    KC::Char(chr) => { match keyevent.kind {
                        KEK::Press => {
                            if let Some(freq) = self.config.get_freq(chr) {
                                self.shared_state.set(chr, freq);
                            }
                        },
                        KEK::Release => self.shared_state.remove(chr),
                        KEK::Repeat => {},
                    }},
                    other => queue!(stdout, crossterm::cursor::MoveTo(0, 0), crossterm::style::Print(format!("{:?}", other)))?,
                }
            }

            // draw
            let shared_state = self.shared_state.cloned();

            const PADDING_LEFT: u16 = 4;
            const PADDING_TOP: u16 = 2;

            const WHITE_KEYS: &[char] = &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '[', ']'];
            const BLACK_KEYS: &[char] = &['1', '2','4', '5', '7', '8', '9', '-', '='];

            let mut white_keys: [PrintKeyCommand; WHITE_KEYS.len()] = [PrintKeyCommand::default(); WHITE_KEYS.len()];
            let mut black_keys: [PrintKeyCommand; BLACK_KEYS.len()] = [PrintKeyCommand::default(); BLACK_KEYS.len()];

            let mut iter_white: usize = 0;
            let mut iter_black: usize = 0;
            for key in self.config.keys_src.iter() {
                use crossterm::style;
                // let x = PADDING_LEFT + (3 * self.config.get_key_index(key)? as u16);
                let x = PADDING_LEFT + (3 * iter_white) as u16;
                if WHITE_KEYS.contains(key) { // print the white key
                    let color = match shared_state.get(key) {
                        Some(_) => style::Color::Grey,
                        None => style::Color::White,
                    };

                    white_keys[iter_white] = PrintKeyCommand {
                        bg_color: color,
                        width: 3,
                        height: 5,
                        pad_top: PADDING_TOP,
                        x_pos: x,
                    };
                    iter_white += 1;
                }
                else if BLACK_KEYS.contains(key) {
                    let x = x-1;
                    let color = match shared_state.get(key) {
                        Some(_) => style::Color::DarkBlue,
                        None => style::Color::Blue,
                    };

                    black_keys[iter_black] = PrintKeyCommand {
                        bg_color: color,
                        width: 2,
                        height: 3,
                        pad_top: PADDING_TOP,
                        x_pos: x,
                    };
                    iter_black += 1;
                }
                else { bail!("Impossible key, neither white nor black") }
            }

            for cmd in white_keys.into_iter() { cmd.queue(&mut stdout)?; }
            for cmd in black_keys.into_iter() { cmd.queue(&mut stdout)?; }

            stdout.flush()?;
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




