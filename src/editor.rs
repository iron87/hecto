use crossterm::event::KeyCode;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::style::Stylize;
use std::io::{self, stdout, Write};
mod terminal;
use terminal::{Position, Terminal};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        self.draw_rows();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
       


    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            Terminal::hide_cursor()?;
            self.refresh_screen()?;
            Terminal::show_cursor()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers,  ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Char(c) => {
                    Terminal::print(&c.to_string());
                    io::stdout().flush().unwrap();
                },
                _ => (),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n");
        } 
        Ok(())
    }
    fn draw_rows(&self) -> Result<(), std::io::Error> {
        let terminal_size =  Terminal::size().unwrap();
        for i in 0..terminal_size.1 {
            if i == terminal_size.1 - 1 {
                Terminal::print("Footer")?;
            }
            else if i == terminal_size.1 / 3 {
                Self::print_welcome_message();
            } else {
                Terminal::print("~\r\n")?;
            }
        }
        let top_left = Position { x: 0, y: 0 };
        Terminal::move_cursor(&top_left)?;
        Ok(())
    }


    fn print_welcome_message() {
        Terminal::print(&Self::center_text(&format!("Hecto editor -- version {}\r\n", env!("CARGO_PKG_VERSION"))));
    }

    fn center_text(text: &str) -> String {
        let terminl_width = Terminal::size().unwrap().0;
        let padding = " ".repeat((terminl_width as usize - text.len()) / 2);
        return format!("{}{}", padding, text);
    }
} 