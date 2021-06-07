use std::io::{Stdout, Write};

use crossterm::{ExecutableCommand, QueueableCommand, Result};

use crate::square_size::{SquareSize, AllowableSizes};

fn draw_lines(sout: &mut Stdout, lines: Vec<&str>, carriage_return_width: u16) -> Result<()> {

    for a in lines.iter() {
        sout.queue(crossterm::style::Print(a))?
            .queue(crossterm::cursor::MoveDown(1))?
            .queue(crossterm::cursor::MoveLeft(carriage_return_width))?;
    }

    Ok(())
}

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

        sout.queue(crossterm::cursor::MoveTo(x, y))?;

        let mut lines = Vec::<&str>::new();

        match (self, &square_size.stated_size) {
            (SquareContents::Blank, AllowableSizes::Size5x5) => {
                lines.push("     ");
                lines.push("     ");
                lines.push("     ");
                lines.push("     ");
                lines.push("     ");
            },
            (SquareContents::Blank, AllowableSizes::Size8x8) => {
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");                    
            }
            (SquareContents::O, AllowableSizes::Size5x5) => {
                lines.push("     ");
                lines.push("  *  ");
                lines.push(" * * ");
                lines.push("  *  ");
                lines.push("     ");
            }
            (SquareContents::O, AllowableSizes::Size8x8) => {
                lines.push("        ");
                lines.push("   **   ");
                lines.push("  *  *  ");
                lines.push(" *    * ");
                lines.push(" *    * ");
                lines.push("  *  *  ");
                lines.push("   **   ");
                lines.push("        ");
            }
            (SquareContents::X, AllowableSizes::Size5x5) => {
                lines.push("     ");
                lines.push(" * * ");
                lines.push("  *  ");
                lines.push(" * * ");
                lines.push("     ");
            }
            (SquareContents::X, AllowableSizes::Size8x8) => {
                    lines.push("        ");
                    lines.push(" *    * ");
                    lines.push("  *  *  ");
                    lines.push("   **   ");
                    lines.push("   **   ");
                    lines.push("  *  *  ");
                    lines.push(" *    * ");
                    lines.push("        ");
            }

        }


        draw_lines(sout, lines, square_size.width)?;

        sout.flush()?;

        Ok(())
        
    }
}