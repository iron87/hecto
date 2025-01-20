use std::io::stdout;

use crossterm::{cursor, event, execute, terminal::{self, Clear, ClearType}, ExecutableCommand};
pub struct Terminal {}


impl Terminal {
    pub fn default() -> Self {
        Terminal {}
    }
    

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
}
