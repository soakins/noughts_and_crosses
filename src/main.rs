mod square_contents;
mod square;
mod square_size;
mod board;

use std::io::Stdout;    // this is Struct representing standard out
use std::io::stdout;    // This function returns a value of the above type

use board::draw_board;
use square::Square;
use square_contents::SquareContents;
use crossterm::Result;
use crossterm::{QueueableCommand, ExecutableCommand};
use square_size::SquareSize;

fn initialise_standard_out_variable() -> Stdout {
    stdout()
}

fn initialise_crossterm(sout: &mut Stdout) -> Result<()>{
    // execute the crossterm command to flip to the alternate screen, squashing the return type unless it's an error, using map
    sout.execute(crossterm::terminal::EnterAlternateScreen).map(|_| ())
}

fn exit_from_crossterm(sout: &mut Stdout) -> Result<()>{
    // execute the crossterm command to flip back from alternate screen, squashing the return type unless it's an error, using map
    sout.execute(crossterm::terminal::LeaveAlternateScreen).map(|_| ())
}

fn main() -> Result<()> {

    let mut sout = initialise_standard_out_variable();

    initialise_crossterm(&mut sout)?;

    let square_size = SquareSize {width: 5, height: 5,};

    draw_board(&mut sout, &square_size)?;

    let top_left = Square::new (
        1,
        1,
        &square_size,
    );

    let mut top_centre = Square::from_another_square(&top_left, square::Directions::East);

    let mut top_right = Square::from_another_square(&top_centre, square::Directions::East);

    let mut middle_left = Square::from_another_square(&top_left, square::Directions::South);

    let mut middle_centre = Square::from_another_square(&top_centre, square::Directions::South);

    let mut middle_right = Square::from_another_square(&top_right, square::Directions::South);

    let mut bottom_left = Square::from_another_square(&middle_left, square::Directions::South);

    let mut bottom_centre = Square::from_another_square(&middle_centre, square::Directions::South);

    let mut bottom_right = Square::from_another_square(&middle_right, square::Directions::South);

    bottom_right.set_contents(SquareContents::O);
    bottom_right.draw_square(&mut sout)?;


    std::thread::sleep(std::time::Duration::from_secs(30));

    exit_from_crossterm(&mut sout)?;

    Ok(())
}
