

pub mod fltk_extensions {

    use fltk::{
        prelude::*,
        enums::Color,
        *
    };
    use std::{
        cell::RefCell,
        // rc::Rc,
    };



    pub struct KeysWrapper {
        pub keys: [RefCell<PianoKey>; 21],
    }

    impl KeysWrapper {

        fn make_key(xval: &mut i32, keyname: &str, white_key: bool) -> PianoKey {
            if white_key {
                *xval += 25;
                return PianoKey::new(25, 55, *xval, 20, keyname, white_key)
            }else {
                // if bldiv == 2 {
                //     return PianoKey::new(5, 13, (10*keynum)+, 20, keyname, white_key)
                // }else {
                //     return PianoKey::new(5, 13, (1*keynum)+5, 20, keyname, white_key)
                // }
                // *xval -= 3;
                return PianoKey::new(15, 45, *xval+17, 20, keyname, white_key)
                
            }
        }

        pub fn new() -> Self {
            // let keytmp = vec!();
            // for i in 
            let mut xval: i32 = 20;
            return Self {
                keys: [
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Ab", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "A", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Bb", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "B", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "C", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Db", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "D", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Eb", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "E", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "F", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Gb", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "G", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Ab", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "A", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Bb", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "B", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "C", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Db", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "D", true)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "Eb", false)),
                    RefCell::from(KeysWrapper::make_key(&mut xval, "E", true)),
                ]
            }
        }
    }

    // impl 

    // const keylist: i32 = 0;

    // #[derive(Copy)]
    pub struct PianoKey {
        inner: widget::Widget,
        // is_on: RefCell<bool>
        is_on: bool,
        color1: Color,
        color2: Color,
        // on: RefCell<bool>,
    }

    impl PianoKey {

        pub fn _is_on(&self) -> bool {
            return self.is_on
        }

        #[inline]
        pub fn set_on(&mut self, value: bool) {
            if self.is_on == value {
                return
            }
            // self.is_on = value;
            self.is_on = value;
            if value {
                let color = self.color2;
                self.set_color(color);
            }else {
                let color = self.color1;
                self.set_color(color);
            }
            self.redraw();
        }

        pub fn new(width: i32, height: i32, x: i32, y: i32, _label: &str, white_key: bool) -> Self {
            let mut inner = widget::Widget::default()
                .with_size(width, height)
                .with_label(_label)
                .with_pos(x, y)
            ;
            inner.set_frame(enums::FrameType::BorderBox);

            /* implement draw function called every frame */
            inner.draw(|widget| {
                draw::draw_box( // draws a rectangular shape
                    widget.frame(), // use widget frame type
                    widget.x(), // x position
                    widget.y(), // y position
                    widget.w(), // width
                    widget.h(), // height
                    widget.color() // color
                );
                draw::set_draw_color(enums::Color::Black); // text color
                draw::set_font(
                    enums::Font::Helvetica,
                    7
                );
                draw::draw_text2(
                    &widget.label(),
                    widget.x(),
                    widget.y(),
                    widget.w(),
                    widget.h(),
                    widget.align()
                );
            });

            inner.handle(|_, _| {
                false
            });

            // handle any event on this widget
            // inner.handle(move |widget, event| match event {
            //     enums::Event::Push => {
            //         *onval.borrow_mut() = true;
            //         widget.do_callback();
            //         return true
            //     },
            //     enums::Event::Released => {
            //         *onval.borrow_mut() = false;
            //         widget.do_callback();
            //         return true
            //     }
            //     _ => return false
            // });

            let mut tmp: Self;

            if white_key {
                tmp =  Self {
                    inner,
                    is_on: false,
                    color1: Color::White,
                    color2: Color::Dark2,
                };
                tmp.set_on(true);
                tmp.set_on(false);
                // tmp.redraw();
                return tmp;
            }else {
                tmp = Self {
                    inner,
                    is_on: false,
                    color1: Color::Dark2,
                    color2: Color::White
                };
                tmp.set_on(true);
                tmp.set_on(false);
                // tmp.redraw();
                return tmp;
            }
        }

    }

    widget_extends!(PianoKey, widget::Widget, inner);

}