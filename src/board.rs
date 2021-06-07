use std::io::{Stdout, Write};
use crossterm::{QueueableCommand, Result}; // trait must be in scope for the queue function to be present on the Stdout
use crate::square_size::SquareSize;

pub fn draw_board(sout: &mut Stdout, square_size: &SquareSize) -> Result<(u16, u16)>{   // returns a tuple of coordinates, where the cursor can be returned to...
    // we need to draw two vertical, and two horizontal lines.
    let left_vert_x = square_size.width + 1;
    let right_vert_x = left_vert_x + square_size.width + 1;
    let vert_min_y = 1u16;
    let vert_max_y = vert_min_y + (square_size.height * 3) + 2;   // two extra to accommodate the two horizontal lines

    let upper_horiz_y = square_size.height + 1;
    let lower_horiz_y = upper_horiz_y + square_size.height + 1;
    let horiz_min_x = 1u16;
    let horiz_max_x = horiz_min_x + (square_size.width * 3) + 2;   // two extra to accommodate the two vertical lines

    for a in vert_min_y .. vert_max_y {
        sout.queue(crossterm::cursor::MoveTo(left_vert_x, a))?
            .queue(crossterm::style::Print("|"))?
            .queue(crossterm::cursor::MoveTo(right_vert_x, a))?
            .queue(crossterm::style::Print("|"))?;
    }

    for a in horiz_min_x .. horiz_max_x {
        sout.queue(crossterm::cursor::MoveTo(a, upper_horiz_y))?
            .queue(crossterm::style::Print("-"))?
            .queue(crossterm::cursor::MoveTo(a, lower_horiz_y))?
            .queue(crossterm::style::Print("-"))?;
    }

    sout.flush()?;

    Ok((horiz_min_x, vert_max_y))
}
