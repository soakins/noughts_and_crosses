use std::io::{Stdout, Write};

use crossterm::{ExecutableCommand, QueueableCommand, Result};

use crate::square_size::SquareSize;

pub enum SquareContents {
    Blank,
    X,
    O,
}
impl Clone for SquareContents {
    fn clone(&self) -> Self {
        match self {
            Self::Blank => SquareContents::Blank,
            Self::O => SquareContents::O,
            Self::X => SquareContents::X,
        }
    }
}
impl SquareContents {
    pub fn draw_square_contents(&self, sout: &mut Stdout, x: u16, y: u16, square_size: &SquareSize) -> Result<()>{
        
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