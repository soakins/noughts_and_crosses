use std::io::{Stdout, Write};

use crossterm::{ExecutableCommand, QueueableCommand, Result};

pub enum SquareContents {
    Blank,
    X,
    O,
}
impl SquareContents {
    pub fn draw_square_contents(&self, sout: &mut Stdout, x: u16, y: u16, width: u16, height: u16) -> Result<()>{
        
        let to_print = match self {
            SquareContents::Blank => " ",
            SquareContents::O => "O",
            SquareContents::X => "X",
        };

        sout.queue(crossterm::cursor::MoveTo(x, y))?
            .queue(crossterm::style::Print(to_print))?;


        sout.flush()?;

        Ok(())
        
    }
}