use std::{fmt::Display, io::Stdout};

use crossterm::{QueueableCommand, Result};

use crate::{square_size::THE_SQUARE_SIZE, CHARACTER_H, CHARACTER_V};

fn draw_lines(sout: &mut Stdout, lines: Vec<&str>, carriage_return_width: u16) -> Result<()> {
    for a in lines.iter() {
        sout.queue(crossterm::style::Print(a))?
            .queue(crossterm::cursor::MoveDown(1))?
            .queue(crossterm::cursor::MoveLeft(carriage_return_width))?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
pub enum SquareContents {
    Blank,
    X,
    O,
}
impl Display for SquareContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            SquareContents::X => "X",
            SquareContents::O => "O",
            SquareContents::Blank => " ",
        };
        write!(f, "'{}'", character)
    }
}
impl PartialEq for SquareContents {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SquareContents::X, SquareContents::X) => true,
            (SquareContents::O, SquareContents::O) => true,
            (SquareContents::Blank, SquareContents::Blank) => true,
            _ => false,
        }
    }
}
impl SquareContents {
    pub fn draw_square_contents(&self, sout: &mut Stdout, x: u16, y: u16) -> Result<()> {
        sout.queue(crossterm::cursor::MoveTo(x, y))?;

        let mut lines = Vec::<&str>::new();

        match (self, THE_SQUARE_SIZE.width, THE_SQUARE_SIZE.height) {
            (SquareContents::Blank, 5, 5) => {
                lines.push("     ");
                lines.push("     ");
                lines.push("     ");
                lines.push("     ");
                lines.push("     ");
            }
            (SquareContents::Blank, 8, 8) => {
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
            }
            (SquareContents::Blank, 9, 9) => {
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("        ");
                lines.push("");
            }
            (SquareContents::O, 5, 5) => {
                lines.push("     ");
                lines.push("  *  ");
                lines.push(" * * ");
                lines.push("  *  ");
                lines.push("     ");
            }
            (SquareContents::O, 8, 8) => {
                lines.push("        ");
                lines.push("   **   ");
                lines.push("  *  *  ");
                lines.push(" *    * ");
                lines.push(" *    * ");
                lines.push("  *  *  ");
                lines.push("   **   ");
                lines.push("        ");
            }
            (SquareContents::X, 5, 5) => {
                lines.push("     ");
                lines.push(" * * ");
                lines.push("  *  ");
                lines.push(" * * ");
                lines.push("     ");
            }
            (SquareContents::X, 8, 8) => {
                lines.push("        ");
                lines.push(" *    * ");
                lines.push("  *  *  ");
                lines.push("   **   ");
                lines.push("   **   ");
                lines.push("  *  *  ");
                lines.push(" *    * ");
                lines.push("        ");
            }
            _ => {
                panic!("Unknown square size!");
            }
        }

        draw_lines(sout, lines, THE_SQUARE_SIZE.width)?;

        Ok(())
    }
}
mod square_content_tests {
    use super::SquareContents;
    #[test]
    fn test_one() {
        let a = SquareContents::O;
        let b = SquareContents::O;
        assert_eq!(a, b);
    }
}
