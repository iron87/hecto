use crossterm::event::KeyCode;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::{cursor, event, execute, ExecutableCommand};
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
            self.refresh_screen()?;
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
                    print!("{}", c);
                    io::stdout().flush().unwrap();
                },
                _ => print!("{:?}\r\n", code),
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
    fn draw_rows(&self) {
        let terminal_size =  Terminal::size().unwrap();
        for _ in 0..terminal_size.1 {
            print!("~\r\n");
        }
        cursor::MoveTo(0, 0);
    }
}