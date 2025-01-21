use crossterm::event::KeyCode;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::style::Stylize;
use std::io::{self, stdout, Write};
mod terminal;
use terminal::Terminal;

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
    fn draw_rows(&self) {
        let terminal_size =  Terminal::size().unwrap();
        for i in 0..terminal_size.1 {
            if i == terminal_size.1 - 1 {
                Terminal::print("Footer");
            } else {
            Terminal::print("~\r\n");
            }
        }
        Terminal::move_cursor(0, 0);
    }
}