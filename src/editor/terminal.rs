use std::io::stdout;

use crossterm::{ cursor, queue, style::Print, terminal::{self,disable_raw_mode, enable_raw_mode, Clear, ClearType}, ExecutableCommand};

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
        queue!(stdout, Clear(ClearType::All))
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        terminal::size()
    }

    pub fn move_cursor(x: u16, y: u16) -> Result<(), std::io::Error> {
       queue!(stdout(),cursor::MoveTo(x, y))?;
       Ok(())

    }
    
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(),cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(),cursor::Show)?;
        Ok(())
    }
    
    pub fn print(arg: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(arg))?;
        Ok(())
    }
}
