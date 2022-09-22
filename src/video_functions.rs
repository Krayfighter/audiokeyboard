

// mod crate::sound_generation;

pub mod video_functions {

    use crate::sound_generation::sound_generation;

    use sound_generation::NoteIndicator;

    use std::sync::{
        // mpsc,
        mpsc::Sender
    };





    #[inline]
    pub fn parse_keyboard_character(character: char, enable: bool, tx: Sender<NoteIndicator>, tx1: Sender<NoteIndicator>) {
        if character == '1' { // D4
            tx.send(
                NoteIndicator::new(0,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(0,true,  enable)
            ).unwrap();
        }else if character == 'q' { // E4
            tx.send(
                NoteIndicator::new(1,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(1,true,  enable)
            ).unwrap();
        }else if character == '2' { // F4
            tx.send(
                NoteIndicator::new(2,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(2,true,  enable)
            ).unwrap();
        }else if character == 'w' { // G4
            tx.send(
                NoteIndicator::new(3,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(3,true,  enable)
            ).unwrap();
        }else if character == 'e' { // A4
            tx.send(
                NoteIndicator::new(4,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(4,true,  enable)
            ).unwrap();
        }else if character == '4' { // B4
            tx.send(
                NoteIndicator::new(5,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(5,true,  enable)
            ).unwrap();
        }else if character == 'r' { // C5
            tx.send(
                NoteIndicator::new(6,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(6,true,  enable)
            ).unwrap();
        }else if character == '5' { // TEST
            tx.send(
                NoteIndicator::new(7,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(7,true,  enable)
            ).unwrap();
        }else if character == 't' { // D4
            tx.send(
                NoteIndicator::new(8,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(8,true,  enable)
            ).unwrap();
        }else if character == 'y' { // E4
            tx.send(
                NoteIndicator::new(9,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(9,true,  enable)
            ).unwrap();
        }else if character == '7' { // F4
            tx.send(
                NoteIndicator::new(10,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(10,true,  enable)
            ).unwrap();
        }else if character == 'u' { // G4
            tx.send(
                NoteIndicator::new(11,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(11,true,  enable)
            ).unwrap();
        }else if character == '8' { // A4
            tx.send(
                NoteIndicator::new(12,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(12,true,  enable)
            ).unwrap();
        }else if character == 'i' { // B4
            tx.send(
                NoteIndicator::new(13,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(13,true,  enable)
            ).unwrap();
        }else if character == '9' { // C5
            tx.send(
                NoteIndicator::new(14,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(14,true,  enable)
            ).unwrap();
        }else if character == 'o' { // TEST
            tx.send(
                NoteIndicator::new(15,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(15,true,  enable)
            ).unwrap();
        }else if character == 'p' { // D4
            tx.send(
                NoteIndicator::new(16,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(16,true,  enable)
            ).unwrap();
        }else if character == '-' { // E4
            tx.send(
                NoteIndicator::new(17,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(17,true,  enable)
            ).unwrap();
        }else if character == '[' { // F4
            tx.send(
                NoteIndicator::new(18,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(18,true,  enable)
            ).unwrap();
        }else if character == '=' { // G4
            tx.send(
                NoteIndicator::new(19,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(19,true,  enable)
            ).unwrap();
        }else if character == ']' { // A4
            tx.send(
                NoteIndicator::new(20,true,  enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(20,true,  enable)
            ).unwrap();
        }

        else if character == 'z' { // D4
            tx.send(
                NoteIndicator::new(0, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(0, false,  enable)
            ).unwrap();
        }else if character == 's' { // E4
            tx.send(
                NoteIndicator::new(1, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(1, false,  enable)
            ).unwrap();
        }else if character == 'x' { // F4
            tx.send(
                NoteIndicator::new(2, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(2, false,  enable)
            ).unwrap();
        }else if character == 'd' { // G4
            tx.send(
                NoteIndicator::new(3, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(3, false,  enable)
            ).unwrap();
        }else if character == 'c' { // A4
            tx.send(
                NoteIndicator::new(4, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(4, false,  enable)
            ).unwrap();
        }else if character == 'f' { // B4
            tx.send(
                NoteIndicator::new(5, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(5, false,  enable)
            ).unwrap();
        }else if character == 'v' { // C5
            tx.send(
                NoteIndicator::new(6, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(6, false,  enable)
            ).unwrap();
        }else if character == 'b' { // TEST
            tx.send(
                NoteIndicator::new(7, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(7, false,  enable)
            ).unwrap();
        }else if character == 'h' { // D4
            tx.send(
                NoteIndicator::new(8, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(8, false,  enable)
            ).unwrap();
        }else if character == 'n' { // E4
            tx.send(
                NoteIndicator::new(9, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(9, false,  enable)
            ).unwrap();
        }else if character == 'j' { // F4
            tx.send(
                NoteIndicator::new(10, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(10, false,  enable)
            ).unwrap();
        }else if character == 'm' { // G4
            tx.send(
                NoteIndicator::new(11, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(11, false,  enable)
            ).unwrap();
        }else if character == ',' { // A4
            tx.send(
                NoteIndicator::new(12, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(12, false,  enable)
            ).unwrap();
        }else if character == 'l' { // B4
            tx.send(
                NoteIndicator::new(13, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(13,true,  enable)
            ).unwrap();
        }else if character == '.' { // C5
            tx.send(
                NoteIndicator::new(14,  false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(14, false,  enable)
            ).unwrap();
        }else if character == ';' { // TEST
            tx.send(
                NoteIndicator::new(15, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(15, false,  enable)
            ).unwrap();
        }else if character == '/' { // TEST
            tx.send(
                NoteIndicator::new(16, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(16, false,  enable)
            ).unwrap();
        }else if character == '\'' { // TEST
            tx.send(
                NoteIndicator::new(17, false, enable)
            ).unwrap();
            tx1.send(
                NoteIndicator::new(17, false,  enable)
            ).unwrap();
        }
        else {}
    }

}