mod square_contents;
mod square;
mod square_size;
mod board;

use std::io::Stdout;    // this is Struct representing standard out
use std::io::stdout;    // This function returns a value of the above type
use std::io::Write;     // Trait brought into scope. Provides the flush for stdout

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
    sout.execute(crossterm::terminal::EnterAlternateScreen).map(|_| ())?;
    crossterm::terminal::enable_raw_mode().map(|_| ())
}

fn exit_from_crossterm(sout: &mut Stdout) -> Result<()>{
    // execute the crossterm command to flip back from alternate screen, squashing the return type unless it's an error, using map
    sout.execute(crossterm::terminal::LeaveAlternateScreen).map(|_| ())?;
    crossterm::terminal::disable_raw_mode().map(|_| ())
}
fn try_moving<'a>(direction: square::Directions, paths: &'a Vec<square::PathsForCursor>, cursor_square: &'a Square) -> Option<&'a Square<'a>> {
    let mut count = 0;
    let mut destination_square_maybe: Option<&Square> = None;
    for a in paths.iter() {
        if std::ptr::eq(a.from, cursor_square) {
            match direction {
                square::Directions::North => {destination_square_maybe = a.to_north;}
                square::Directions::South => {destination_square_maybe = a.to_south;}
                square::Directions::East => {destination_square_maybe = a.to_east;}
                square::Directions::West => {destination_square_maybe = a.to_west;}
            }
        }
        count += 1;
    }
    destination_square_maybe
}

fn main() -> Result<()> {

    let mut sout = initialise_standard_out_variable();

    initialise_crossterm(&mut sout)?;

    let square_size = SquareSize::from_allowable_size(square_size::AllowableSizes::Size8x8);

    let system_cursor_pos = draw_board(&mut sout, &square_size)?;  // function returns a tuple of coords

    let mut top_left = Square::new (
        1,
        1,
        &square_size,
    );

    let mut top_centre = top_left.spawn_another_square(square::Directions::East);

    let mut top_right = top_centre.spawn_another_square(square::Directions::East);

    let mut middle_left = top_left.spawn_another_square(square::Directions::South);

    let mut middle_centre = top_centre.spawn_another_square(square::Directions::South);

    let mut middle_right = top_right.spawn_another_square(square::Directions::South);

    let mut bottom_left = middle_left.spawn_another_square(square::Directions::South);

    let mut bottom_centre = middle_centre.spawn_another_square(square::Directions::South);

    let mut bottom_right = middle_right.spawn_another_square(square::Directions::South);

    let mut paths: Vec<square::PathsForCursor> = Vec::new();

    paths.push(square::PathsForCursor{from: &top_left, to_north: None, to_south: Some(&middle_left), to_east: Some(&top_centre), to_west: None});
    paths.push(square::PathsForCursor{from: &top_centre, to_north: None, to_south: Some(&middle_centre), to_east: Some(&top_right), to_west: Some(&top_left)});
    paths.push(square::PathsForCursor{from: &top_right, to_north: None, to_south: Some(&middle_right), to_east: None, to_west: Some(&top_centre)});
    paths.push(square::PathsForCursor{from: &middle_left, to_north: Some(&top_left), to_south: Some(&bottom_left), to_east: Some(&middle_centre), to_west: None});
    paths.push(square::PathsForCursor{from: &middle_centre, to_north: Some(&top_centre), to_south: Some(&bottom_centre), to_east: Some(&middle_right), to_west: Some(&middle_left)});
    paths.push(square::PathsForCursor{from: &middle_right, to_north: Some(&top_right), to_south: Some(&bottom_right), to_east: None, to_west: Some(&middle_centre)});
    paths.push(square::PathsForCursor{from: &bottom_left, to_north: Some(&middle_left), to_south: None, to_east: Some(&bottom_centre), to_west: None});
    paths.push(square::PathsForCursor{from: &bottom_centre, to_north: Some(&middle_centre), to_south: None, to_east: Some(&bottom_right), to_west: Some(&bottom_left)});
    paths.push(square::PathsForCursor{from: &bottom_right, to_north: Some(&middle_right), to_south: None, to_east: None, to_west: Some(&bottom_centre)});

    /*top_left.set_neighbour(square::Directions::East, &top_centre);
    top_left.set_neighbour(square::Directions::South, &middle_left);

    top_centre.set_neighbour(square::Directions::East, &top_right);
    top_centre.set_neighbour(square::Directions::South, &middle_centre);
    top_centre.set_neighbour(square::Directions::West, &top_left);

    top_right.set_neighbour(square::Directions::South, &middle_right);
    top_right.set_neighbour(square::Directions::West, &top_centre);*/

    /*bottom_right.set_contents(SquareContents::O);
    bottom_right.draw_square(&mut sout)?;

    bottom_left.set_contents(SquareContents::X);
    bottom_left.draw_square(&mut sout)?;*/

    let mut cursor_square = &middle_left;
    let mut destination_square: Option<&Square> = None;

    let mut keep_looping = true;

    let polling_duration = std::time::Duration::from_millis(250);

    while keep_looping {
        
        match crossterm::event::poll(polling_duration) {
            Ok(true) => {
                match crossterm::event::read()? {
                    crossterm::event::Event::Key(kev) => {
                
                        let key_code = kev.code;
                        match key_code {
/*                            crossterm::event::KeyCode::Char(c) => {
                                match c {
                                    'o' => {cursor_square.set_contents(SquareContents::O);}
                                    'x' => {cursor_square.set_contents(SquareContents::X);}
                                    ' ' => {cursor_square.set_contents(SquareContents::Blank);}
                                    _ => {}
                                }
                            }*/
                            crossterm::event::KeyCode::Esc => {keep_looping = false;}
                            crossterm::event::KeyCode::Up => {destination_square = try_moving(square::Directions::North, &paths, &cursor_square);}
                            crossterm::event::KeyCode::Down => {destination_square = try_moving(square::Directions::South, &paths, &cursor_square);}
                            crossterm::event::KeyCode::Left => {destination_square = try_moving(square::Directions::West, &paths, &cursor_square);}
                            crossterm::event::KeyCode::Right => {destination_square = try_moving(square::Directions::East, &paths, &cursor_square);}
                            _ => {}
                        }
                    }
                    _ => {}
                }
                if destination_square.is_some() {
                    cursor_square.draw_square(&mut sout)?;   // will rub out current game cursor
                    cursor_square = destination_square.unwrap();
                    destination_square = None
                }
            }
            _ => {}
        }

        // draw the game cursor

        sout.queue(crossterm::cursor::MoveTo(cursor_square.screen_x, cursor_square.screen_y))?
            .queue(crossterm::style::Print("\u{250C}"))?
            .queue(crossterm::cursor::MoveTo(cursor_square.screen_x + square_size.width - 1, cursor_square.screen_y))?
            .queue(crossterm::style::Print("\u{2510}"))?
            .queue(crossterm::cursor::MoveTo(cursor_square.screen_x, cursor_square.screen_y + square_size.height - 1))?
            .queue(crossterm::style::Print("\u{2514}"))?
            .queue(crossterm::cursor::MoveTo(cursor_square.screen_x + square_size.width - 1, cursor_square.screen_y + square_size.height - 1))?
            .queue(crossterm::style::Print("\u{2518}"))?;

        sout.flush()?;


        // move the system cursor to somewhere that it can remain every time.
        sout.queue(crossterm::cursor::MoveTo(system_cursor_pos.0, system_cursor_pos.1))?
            .queue(crossterm::style::Print("            "))?
            .queue(crossterm::cursor::MoveTo(system_cursor_pos.0, system_cursor_pos.1))?;

        sout.flush()?;

    }

    exit_from_crossterm(&mut sout)?;

    Ok(())
}
