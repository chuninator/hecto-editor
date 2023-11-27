use std::io:: {self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {

    //Termion gives us access to Key::Char, Key::Ctrl, and Key::Alt
    //Termion also handles the character instead of printing btyes to chars.
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys(){
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({})\r", c as u8, c);
                        }

                        if c == 'q' {
                            break;
                        }
                    }

                    Key::Ctrl('q') => break, 
                    _ => println!("{:?}\r", key),
                },
                Err(err) => die(err),
            }


        }

   
    }

    pub fn default() -> Self {
        Self{}
    }

}

//Error handle
fn die(e:std::io::Error) {
    panic!("{}", e);
}