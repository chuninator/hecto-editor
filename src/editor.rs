use std::io:: {self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool, 
}

impl Editor {
    pub fn run(&mut self) {

    //Termion gives us access to Key::Char, Key::Ctrl, and Key::Alt
    //Termion also handles the character instead of printing btyes to chars.
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
            if self.should_quit {
                break; 
            }
        }
    }


    pub fn default() -> Self {
        Self{should_quit: false}
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Char('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}


fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

//Error handle
fn die(e:std::io::Error) {
    panic!("{}", e);
}