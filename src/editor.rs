use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize, 
    pub y: usize,
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn run(&mut self) {
        //Termion gives us access to Key::Char, Key::Ctrl, and Key::Alt
        //Termion also handles the character instead of printing btyes to chars.
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to init terminal"),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize; 
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }


    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                //[..width] is slicing the string from its beginning until width has been calculated as the min screen size
                //or the welcome message length never slicing more of a string than what is there. 
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position {x: 0, y: 0});

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('f') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
}

//Error handle
fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
