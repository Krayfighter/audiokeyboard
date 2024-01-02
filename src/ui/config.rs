

use std::collections::HashMap;


const DEFAULT_KEYS: &[char] = &[
    '1', 'q', '2', 'w', 'e', '4', 'r', '5', 't', 'y', '7', 'u', '8', 'i', '9', 'o', 'p', '-', '[', '=', ']'
];
const DEFAULT_KEYS_EXTENDED: &[char] = &[
    'a', 'z', 'x', 'd', 'c', 'f', 'v', 'g', 'b', 'n', 'j', 'm', 'k',
    ',', '.', ';', '/', '1', 'q', '2', 'w', 'e', '4', 'r', '5', 't',
    'y', '7', 'u', '8', 'i', '9', 'o', 'p', '-', '[', '=', ']'
];

const DEFAULT_KEYS_SETTINGS: (&[char], u16, u16) = (
    DEFAULT_KEYS,
    48,
    1
);

const DEFAULT_KEYS_EXTENDED_SETTINGS: (&[char], u16, u16) = (
    DEFAULT_KEYS_EXTENDED,
    31,
    6,
);

// white = true, black = false
type KeyColor = bool;
pub const WHITE_KEY: bool = true;
pub const BLACK_KEY: bool = false;

pub struct KeySet {
    pub keys: Box<[char]>,
    pub map: HashMap<char, (u16, KeyColor)>,
    pub temperament: f32,
}

impl Default for KeySet {
    fn default() -> Self {
        return Self::new(
            DEFAULT_KEYS_SETTINGS.0,
            DEFAULT_KEYS_SETTINGS.1,
            DEFAULT_KEYS_SETTINGS.2,
        );
    }
}

impl KeySet {

    fn keycolor(
        base: u16,
        keynum: u16,
        // the number of half-steps above C
        base_note_C_offset: u16
    ) -> KeyColor {
        const black_key_degrees: [i32 ; 5] = [
            1, 4, 6, 9, 11];
        return black_key_degrees.contains(&(
            ((
                keynum as i32 -
                base as i32 -
                base_note_C_offset as i32
            ).abs() % 12)
        )) == false;
    }

    fn generate_hashmap(
        keys: &[char],
        bottom: u16,
        offset: u16,
    ) -> HashMap<char, (u16, KeyColor)> {
        let values = (bottom..(keys.len() as u16 + bottom))
            .into_iter()
            .map(|keynum| (keynum, Self::keycolor(bottom, keynum, offset)))
            .collect::<Vec<(u16, bool)>>();
        std::fs::write("test.txt", format!("{:?}", &values));
        return keys
            .into_iter()
            .cloned()
            .zip(values)
            .into_iter()
            .collect();
    }

    pub fn new(keys: &[char], bottom: u16, offset: u16) -> Self {
        return Self {
            map: Self::generate_hashmap(keys, bottom, offset),
            keys: Box::from(keys),
            temperament: 12.,
        }
    }

    pub fn freq(&self, key: char) -> Option<f32> {
        const MIDDLE_A: f32 = 440.;
        const MIDDLE_A_KEY_NUMBER: i32 = 49;

        if let Some(key_number) = self.map.get(&key) {
            return Some ( MIDDLE_A * 2.0f32.powf(
                (key_number.0 as i32 - MIDDLE_A_KEY_NUMBER) as f32 /
                self.temperament
            ) );
        }
        else { return None; }
    }

    fn get_key_index(&self, key: &char) -> anyhow::Result<usize> {
        return self.keys.iter()
            .position(|lhs_key| lhs_key == key)
            .ok_or(anyhow!("unable to find key position"));
    }

    pub fn keysets() -> Vec<String> {
        return vec![
            "default".to_string(),
            "default extended".to_string()
        ];
    }

    pub fn set_keyset(&mut self, name: String) -> anyhow::Result<()> {
        match name.as_str() {
            "default" => {
                self.keys = Box::from(DEFAULT_KEYS_SETTINGS.0);
                self.map = Self::generate_hashmap(
                    DEFAULT_KEYS_SETTINGS.0,
                    DEFAULT_KEYS_SETTINGS.1,
                    DEFAULT_KEYS_SETTINGS.2
                );
            },
            "default extended" => {
                self.keys = Box::from(DEFAULT_KEYS_EXTENDED_SETTINGS.0);
                self.map = Self::generate_hashmap(
                    DEFAULT_KEYS_EXTENDED_SETTINGS.0,
                    DEFAULT_KEYS_EXTENDED_SETTINGS.1,
                    DEFAULT_KEYS_EXTENDED_SETTINGS.2,
                );
            },
            _ => bail!("unable to match given string to a keyset"),
        }
        return Ok(());
    }
}





