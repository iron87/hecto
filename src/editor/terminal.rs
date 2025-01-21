use std::io::stdout;

use crossterm::{ execute, terminal::{self,disable_raw_mode, enable_raw_mode, Clear, ClearType}, ExecutableCommand};

pub struct Terminal {}


impl Terminal {
    pub fn default() -> Self {
        Terminal {}
    }
    

    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        terminal::size()
    }
}
