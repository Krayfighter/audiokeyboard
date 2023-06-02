
use crate::sound_types;

use eframe::egui;

use std::sync::mpsc;

use std::collections::HashSet;


pub struct AppState {
    tx: mpsc::Sender<(sound_types::Note, bool)>, // transmit keyboard state to the audio thread
    pressed_keys: HashSet<egui::Key>,
}

impl AppState {
    pub fn new(tx: mpsc::Sender<(sound_types::Note, bool)>) -> Self {
        return Self {
            tx,
            pressed_keys: HashSet::new(),
        }
    }

    fn send_note(&self, note: sound_types::Note, value: bool) -> () {
        let send_result = self.tx.send((
            note,
            value
        ));

        match send_result {
            Ok(_) => {},
            Err(_) => {
                println!("I'm not sure what happened but the was an error sending a note");
            }
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            println!("{:?}", self.pressed_keys); // TODO remove for non-debug


            // C3
            if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
                if !self.pressed_keys.contains(&egui::Key::Q) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::C,3),true);
                    self.pressed_keys.insert(egui::Key::Q);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::Q)) {
                self.send_note(sound_types::Note::new(sound_types::TET::C,3),false);
                self.pressed_keys.remove(&egui::Key::Q);
            }
            // Db3
            if ctx.input(|i| i.key_pressed(egui::Key::Num2)) {
                if !self.pressed_keys.contains(&egui::Key::Num2) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::Db,3),true);
                    self.pressed_keys.insert(egui::Key::Num2);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::Num2)) {
                self.send_note(sound_types::Note::new(sound_types::TET::Db,3),false);
                self.pressed_keys.remove(&egui::Key::Num2);
            }
            // D3
            if ctx.input(|i| i.key_pressed(egui::Key::W)) {
                if !self.pressed_keys.contains(&egui::Key::W) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::D,3),true);
                    self.pressed_keys.insert(egui::Key::W);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::W)) {
                self.send_note(sound_types::Note::new(sound_types::TET::D,3),false);
                self.pressed_keys.remove(&egui::Key::W);
            }
            // Eb3
            if ctx.input(|i| i.key_pressed(egui::Key::Num3)) {
                if !self.pressed_keys.contains(&egui::Key::Num3) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::Eb,3),true);
                    self.pressed_keys.insert(egui::Key::Num3);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::Num3)) {
                self.send_note(sound_types::Note::new(sound_types::TET::Eb,3),false);
                self.pressed_keys.remove(&egui::Key::Num3);
            }
            // E3
            if ctx.input(|i| i.key_pressed(egui::Key::E)) {
                if !self.pressed_keys.contains(&egui::Key::E) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::E,3),true);
                    self.pressed_keys.insert(egui::Key::E);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::E)) {
                self.send_note(sound_types::Note::new(sound_types::TET::E,3),false);
                self.pressed_keys.remove(&egui::Key::E);
            }
            // F3
            if ctx.input(|i| i.key_pressed(egui::Key::R)) {
                if !self.pressed_keys.contains(&egui::Key::R) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::F,3),true);
                    self.pressed_keys.insert(egui::Key::R);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::R)) {
                self.send_note(sound_types::Note::new(sound_types::TET::F,3),false);
                self.pressed_keys.remove(&egui::Key::R);
            }
            // Gb3
            if ctx.input(|i| i.key_pressed(egui::Key::Num5)) {
                if !self.pressed_keys.contains(&egui::Key::Num5) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::Gb,3),true);
                    self.pressed_keys.insert(egui::Key::Num5);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::Num5)) {
                self.send_note(sound_types::Note::new(sound_types::TET::Gb,3),false);
                self.pressed_keys.remove(&egui::Key::Num5);
            }
            // G3
            if ctx.input(|i| i.key_pressed(egui::Key::T)) {
                if !self.pressed_keys.contains(&egui::Key::T) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::G,3),true);
                    self.pressed_keys.insert(egui::Key::T);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::T)) {
                self.send_note(sound_types::Note::new(sound_types::TET::G,3),false);
                self.pressed_keys.remove(&egui::Key::T);
            }
            // Ab3
            if ctx.input(|i| i.key_pressed(egui::Key::Num6)) {
                if !self.pressed_keys.contains(&egui::Key::Num6) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::Ab,3),true);
                    self.pressed_keys.insert(egui::Key::Num6);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::Num6)) {
                self.send_note(sound_types::Note::new(sound_types::TET::Ab,3),false);
                self.pressed_keys.remove(&egui::Key::Num6);
            }
            // A4
            if ctx.input(|i| i.key_pressed(egui::Key::Y)) {
                if !self.pressed_keys.contains(&egui::Key::Y) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::A,3),true);
                    self.pressed_keys.insert(egui::Key::Y);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::Y)) {
                self.send_note(sound_types::Note::new(sound_types::TET::A,3),false);
                self.pressed_keys.remove(&egui::Key::Y);
            }
            // Bb4
            if ctx.input(|i| i.key_pressed(egui::Key::Num7)) {
                if !self.pressed_keys.contains(&egui::Key::Num7) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::Bb,3),true);
                    self.pressed_keys.insert(egui::Key::Num7);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::Num7)) {
                self.send_note(sound_types::Note::new(sound_types::TET::Bb,3),false);
                self.pressed_keys.remove(&egui::Key::Num7);
            }
            // B4
            if ctx.input(|i| i.key_pressed(egui::Key::U)) {
                if !self.pressed_keys.contains(&egui::Key::U) { // we do this so that we minimize Atomic calls from mpsc
                    self.send_note(sound_types::Note::new(sound_types::TET::B,3),true);
                    self.pressed_keys.insert(egui::Key::U);
                }
            }
            if ctx.input(|i| i.key_released(egui::Key::U)) {
                self.send_note(sound_types::Note::new(sound_types::TET::B,3),false);
                self.pressed_keys.remove(&egui::Key::U);
            }
            

            // exits the app
            if ctx.input(|key| key.key_pressed(egui::Key::Escape)) {
                todo!(); // this haappens to work at closing the app
            }

        });
    }
}



