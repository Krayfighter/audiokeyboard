

use std::io::Write;
use crossterm::QueueableCommand;


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

fn poll_keyevent(millis: u64) -> anyhow::Result<Option<crossterm::event::KeyEvent>> {
    crossterm::event::poll(std::time::Duration::from_millis(millis))?;
    if let crossterm::event::Event::Key(keyevent) = crossterm::event::read()? {
        return Ok(Some(keyevent));
    } else {
        return Ok(None);
    }
}

pub fn main_menu(
    keyset: &crate::ui::config::KeySet,
    mut shared_state: crate::state::KeyboardState,
    menu_state: &mut super::MenuState,
) -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();
    'menu: loop {
        let keyevent = match poll_keyevent(50) {
            Ok(Some(event)) => event,
            Ok(None) | Err(_) => continue 'menu,
        };

        use crossterm::event::KeyCode as KC;
        use crossterm::event::KeyEventKind as KEK;
        match keyevent.code {
            KC::Esc => {
                *menu_state = super::MenuState::None;
                return Ok(());
            }
            KC::Char(chr) => {
                match keyevent.kind {
                    KEK::Press => {
                        if let Some(freq) = keyset.freq(chr) {
                            shared_state.set(chr, freq);
                        }
                    },
                    KEK::Release => shared_state.remove(chr),
                    _ => {},
                }
            },
            KC::F(1) => {
                *menu_state = super::MenuState::Select(super::SelectMenu::KeySet);
                return Ok(());
            },
            _ => {},
        }

        let state = shared_state.cloned();

        const PADDING_LEFT: u16 = 4;
        const PADDING_TOP: u16 = 2;

        let mut white_key_queue: Vec<PrintKeyCommand> = vec!();
        white_key_queue.reserve(keyset.keys.len()*3 / 4);
        let mut black_key_queue: Vec<PrintKeyCommand> = vec!();
        black_key_queue.reserve(keyset.keys.len() / 2);

        let mut iter_white = 0;
        for key in keyset.keys.iter() {
            let (_knum, kcolor) = keyset.map.get(key).ok_or(
                anyhow!("unable to fetch key in hashmap")
            )?;
            use crossterm::style;
            use crate::ui::config::{WHITE_KEY, BLACK_KEY};

            let x = PADDING_LEFT + (3 * iter_white);
            if *kcolor == WHITE_KEY {
                let color = match state.get(key) {
                    Some(_) => style::Color::Rgb { r: 0xaa, g: 0xaa, b: 0xaa },
                    None => style::Color::Rgb { r: 0xff, g: 0xff, b: 0xff },
                };
                white_key_queue.push( PrintKeyCommand {
                    bg_color: color,
                    height: 5, width: 3,
                    pad_top: PADDING_TOP, x_pos: x
                } );
                iter_white += 1;
            }
            else if *kcolor == BLACK_KEY {
                let color = match state.get(key) {
                    Some(_) => style::Color::Rgb { r: 0x11, g: 0x11, b: 0x11 },
                    None => style::Color::Rgb { r: 0x33, g: 0x33, b: 0x33 },
                };
                black_key_queue.push( PrintKeyCommand {
                    bg_color: color,
                    height: 3, width: 2,
                    pad_top: PADDING_TOP, x_pos: x-1
                });
            }else {
                bail!("impossible key neither black nor white");
            }
        }
        
        for cmd in white_key_queue.into_iter() { cmd.queue(&mut stdout)? }
        for cmd in black_key_queue.into_iter() { cmd.queue(&mut stdout)? }

        stdout.flush()?;

        continue 'menu;
    }
}

pub fn keyset_menu(keyset: &mut super::config::KeySet) -> anyhow::Result<()> {
    let mut menu_index: usize = 0;

    let keysets: Vec<String> = super::config::KeySet::keysets();

    const FG: crossterm::style::Color = crossterm::style::Color::White;
    const BG: crossterm::style::Color = crossterm::style::Color::Black;

    let mut stdout = std::io::stdout();
    'menu: loop {

        for (iter, stringref) in keysets.iter().enumerate() {
            let (fg, bg) = match iter==menu_index {
                true => (&BG, &FG),
                false => (&FG, &BG),
            };
            queue!(stdout,
                crossterm::cursor::MoveTo(0, iter as u16),
                crossterm::style::SetForegroundColor( *fg ),
                crossterm::style::SetBackgroundColor( *bg ),
                crossterm::style::Print( stringref ),
                crossterm::style::ResetColor,
            )?;
        }

        stdout.flush()?;

        let keyevent = match poll_keyevent(50) {
            Ok(Some(event)) => event,
            Ok(None) | Err(_) => continue 'menu,
        };

        use crossterm::event::KeyCode as KC;
        use crossterm::event::KeyEventKind as KEK;
        match keyevent.code {
            KC::Esc => {},
            KC::Up => { match keyevent.kind {
                KEK::Press | KEK::Repeat => {
                    if menu_index == 0 { menu_index = keysets.len()-1; }
                    else { menu_index -= 1; }
                    continue 'menu;
                },
                _ => continue 'menu,
            }},
            KC::Down => { match keyevent.kind {
                KEK::Press | KEK::Repeat => {
                    if menu_index+1 == keysets.len() { menu_index = 0; }
                    else { menu_index += 1; }
                    continue 'menu;
                }
                _ => continue 'menu,
            }},
            KC::Enter => {
                *keyset = super::config::KeySet::from_keyset(
                    keysets[menu_index].clone()
                )?;
            },
            _ => continue 'menu,
        }

        return Ok(());
    }
}



