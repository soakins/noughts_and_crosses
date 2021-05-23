mod square_contents;
mod square;

use std::io::Stdout;    // this is Struct representing standard out
use std::io::stdout;    // This function returns a value of the above type

use square::Square;
use square_contents::SquareContents;
use crossterm::Result;
use crossterm::{QueueableCommand, ExecutableCommand};

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

    let a_square = Square::new (
        SquareContents::Blank,
        10,
        10,
        1,
        1,
    );

    let b_square = Square::new (
        SquareContents::X,
        11,
        11,
        1,
        1,
    );

    let c_square = Square::new (
        SquareContents::O,
        12,
        12,
        1,
        1,
    );

    a_square.draw_square(&mut sout)?;
    b_square.draw_square(&mut sout)?;
    c_square.draw_square(&mut sout)?;

    std::thread::sleep(std::time::Duration::from_secs(10));

    exit_from_crossterm(&mut sout)?;

    Ok(())
}
