mod board;
mod player_names_and_turns_organiser;
mod screen_coords;
mod square;
mod square_contents;
mod square_size;
mod winning_line_coords_for_squares;
mod winning_lines;

use std::io::stdout; // This function returns a value of the above type
use std::io::Stdout; // this is Struct representing standard out
use std::io::Write; // Trait brought into scope. Provides the flush for stdout

use board::draw_board;
use crossterm::Result;
use crossterm::{ExecutableCommand, QueueableCommand};
use square::Square;
use square_contents::SquareContents;
use square_size::THE_SQUARE_SIZE;
use winning_lines::WinningLineDirections;

const CHARACTER_H: char = '\u{255C}';
const CHARACTER_V: char = '\u{2551}';

fn initialise_standard_out_variable() -> Stdout {
    stdout()
}

fn initialise_crossterm(sout: &mut Stdout) -> Result<()> {
    // execute the crossterm command to flip to the alternate screen, squashing the return type unless it's an error, using map
    sout.execute(crossterm::terminal::EnterAlternateScreen)
        .map(|_| ())?;
    crossterm::terminal::enable_raw_mode().map(|_| ())
}

fn exit_from_crossterm(sout: &mut Stdout) -> Result<()> {
    // execute the crossterm command to flip back from alternate screen, squashing the return type unless it's an error, using map
    sout.execute(crossterm::terminal::LeaveAlternateScreen)
        .map(|_| ())?;
    crossterm::terminal::disable_raw_mode().map(|_| ())
}
fn try_moving<'a>(
    direction: square::Directions,
    paths: &'a Vec<square::PathsForCursor>,
    cursor_square: usize,
) -> Option<usize> {
    let mut destination_square_maybe: Option<usize> = None;
    // Find a reference to a square in the squares Vector, using the usize index passed in...

    for a in paths.iter() {
        if a.from == cursor_square {
            match direction {
                square::Directions::North => {
                    destination_square_maybe = a.to_north;
                }
                square::Directions::South => {
                    destination_square_maybe = a.to_south;
                }
                square::Directions::East => {
                    destination_square_maybe = a.to_east;
                }
                square::Directions::West => {
                    destination_square_maybe = a.to_west;
                }
            }
            break;
        }
    }
    destination_square_maybe
}

fn main() -> Result<()> {
    let mut sout = initialise_standard_out_variable();

    initialise_crossterm(&mut sout)?;

    let system_cursor_pos = draw_board(&mut sout)?; // function returns a tuple of coords
    let mut all_squares: Vec<Square> = Vec::new();
    let mut paths: Vec<square::PathsForCursor> = Vec::new();
    let mut winning_lines = winning_lines::WinningLines::new();
    let mut winning_line_coords = winning_line_coords_for_squares::WinningLineCoords::new();

    {
        let top_left_square = Square::new(1, 1, &mut winning_line_coords);
        let top_centre_square = top_left_square
            .spawn_another_square(square::Directions::East, &mut winning_line_coords);
        let top_right_square = top_centre_square
            .spawn_another_square(square::Directions::East, &mut winning_line_coords);
        let middle_left_square = top_left_square
            .spawn_another_square(square::Directions::South, &mut winning_line_coords);
        let middle_centre_square = top_centre_square
            .spawn_another_square(square::Directions::South, &mut winning_line_coords);
        let middle_right_square = top_right_square
            .spawn_another_square(square::Directions::South, &mut winning_line_coords);
        let bottom_left_square = middle_left_square
            .spawn_another_square(square::Directions::South, &mut winning_line_coords);
        let bottom_centre_square = middle_centre_square
            .spawn_another_square(square::Directions::South, &mut winning_line_coords);
        let bottom_right_square = middle_right_square
            .spawn_another_square(square::Directions::South, &mut winning_line_coords);

        all_squares.push(top_left_square); // index 0
        all_squares.push(top_centre_square);
        all_squares.push(top_right_square);
        all_squares.push(middle_left_square);
        all_squares.push(middle_centre_square);
        all_squares.push(middle_right_square);
        all_squares.push(bottom_left_square);
        all_squares.push(bottom_centre_square);
        all_squares.push(bottom_right_square); // index 8

        let top_left_square: usize = 0;
        let top_centre_square: usize = 1;
        let top_right_square: usize = 2;
        let middle_left_square: usize = 3;
        let middle_centre_square: usize = 4;
        let middle_right_square: usize = 5;
        let bottom_left_square: usize = 6;
        let bottom_centre_square: usize = 7;
        let bottom_right_square: usize = 8;

        paths.push(square::PathsForCursor {
            from: top_left_square,
            to_north: None,
            to_south: Some(middle_left_square),
            to_east: Some(top_centre_square),
            to_west: None,
        });
        paths.push(square::PathsForCursor {
            from: top_centre_square,
            to_north: None,
            to_south: Some(middle_centre_square),
            to_east: Some(top_right_square),
            to_west: Some(top_left_square),
        });
        paths.push(square::PathsForCursor {
            from: top_right_square,
            to_north: None,
            to_south: Some(middle_right_square),
            to_east: None,
            to_west: Some(top_centre_square),
        });
        paths.push(square::PathsForCursor {
            from: middle_left_square,
            to_north: Some(top_left_square),
            to_south: Some(bottom_left_square),
            to_east: Some(middle_centre_square),
            to_west: None,
        });
        paths.push(square::PathsForCursor {
            from: middle_centre_square,
            to_north: Some(top_centre_square),
            to_south: Some(bottom_centre_square),
            to_east: Some(middle_right_square),
            to_west: Some(middle_left_square),
        });
        paths.push(square::PathsForCursor {
            from: middle_right_square,
            to_north: Some(top_right_square),
            to_south: Some(bottom_right_square),
            to_east: None,
            to_west: Some(middle_centre_square),
        });
        paths.push(square::PathsForCursor {
            from: bottom_left_square,
            to_north: Some(middle_left_square),
            to_south: None,
            to_east: Some(bottom_centre_square),
            to_west: None,
        });
        paths.push(square::PathsForCursor {
            from: bottom_centre_square,
            to_north: Some(middle_centre_square),
            to_south: None,
            to_east: Some(bottom_right_square),
            to_west: Some(bottom_left_square),
        });
        paths.push(square::PathsForCursor {
            from: bottom_right_square,
            to_north: Some(middle_right_square),
            to_south: None,
            to_east: None,
            to_west: Some(bottom_centre_square),
        });

        winning_lines.add_a_line(
            top_left_square,
            top_centre_square,
            top_right_square,
            WinningLineDirections::Horizontal,
        );
        winning_lines.add_a_line(
            middle_left_square,
            middle_centre_square,
            middle_right_square,
            WinningLineDirections::Horizontal,
        );
        winning_lines.add_a_line(
            bottom_left_square,
            bottom_centre_square,
            bottom_right_square,
            WinningLineDirections::Horizontal,
        );
        winning_lines.add_a_line(
            top_left_square,
            middle_left_square,
            bottom_left_square,
            WinningLineDirections::Vertical,
        );
        winning_lines.add_a_line(
            top_centre_square,
            middle_centre_square,
            bottom_centre_square,
            WinningLineDirections::Vertical,
        );
        winning_lines.add_a_line(
            top_right_square,
            middle_right_square,
            bottom_right_square,
            WinningLineDirections::Vertical,
        );
        winning_lines.add_a_line(
            top_left_square,
            middle_centre_square,
            bottom_right_square,
            WinningLineDirections::TopLeftBottomRight,
        );
        winning_lines.add_a_line(
            top_right_square,
            middle_centre_square,
            bottom_left_square,
            WinningLineDirections::TopRightBottomLeft,
        );
    }

    let mut cursor_square: usize = 0;
    let mut destination_square: Option<usize> = None;
    let mut players = player_names_and_turns_organiser::PlayerNames::new(
        String::from("me"),
        SquareContents::X,
        String::from("you"),
        SquareContents::O,
    );

    players.next_player(); // this nudges the player name organiser onto the first turn.

    let mut keep_looping = true;

    while keep_looping {
        // draw the game cursor

        let sq = all_squares
            .get(cursor_square)
            .expect("Again, we should have found a square.");
        sout.queue(crossterm::cursor::MoveTo(
            sq.screen_coords.x,
            sq.screen_coords.y,
        ))?
        .queue(crossterm::style::Print("\u{250C}"))?
        .queue(crossterm::cursor::MoveTo(
            sq.screen_coords.x + THE_SQUARE_SIZE.width - 1,
            sq.screen_coords.y,
        ))?
        .queue(crossterm::style::Print("\u{2510}"))?
        .queue(crossterm::cursor::MoveTo(
            sq.screen_coords.x,
            sq.screen_coords.y + THE_SQUARE_SIZE.height - 1,
        ))?
        .queue(crossterm::style::Print("\u{2514}"))?
        .queue(crossterm::cursor::MoveTo(
            sq.screen_coords.x + THE_SQUARE_SIZE.width - 1,
            sq.screen_coords.y + THE_SQUARE_SIZE.height - 1,
        ))?
        .queue(crossterm::style::Print("\u{2518}"))?;

        // move the system cursor to somewhere that it can remain every time.
        sout.queue(crossterm::cursor::MoveTo(
            system_cursor_pos.0,
            system_cursor_pos.1,
        ))?
        .queue(crossterm::style::Print("                      "))?
        .queue(crossterm::cursor::MoveTo(
            system_cursor_pos.0,
            system_cursor_pos.1,
        ))?
        .queue(crossterm::style::Print(players.prompt_string()))?;

        sout.flush()?;
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) => {
                match key_event.code {
                    //  there are modifiers available, but I don't think we care about those?? Maybe capital letters are a different char?
                    crossterm::event::KeyCode::Char(c) => {
                        match "OoXx ".find(c) {
                            None => {} // the character pressed was not one we're interested in...
                            Some(_) => {
                                let sq = all_squares.get_mut(cursor_square).expect("Oh no!");
                                match c {
                                    /*
                                    'o' => {
                                        sq.set_contents(SquareContents::O);
                                    }
                                    'x' => {
                                        sq.set_contents(SquareContents::X);
                                    }
                                    */
                                    ' ' => {
                                        sq.set_contents(players.last_player().player_mark);
                                    }
                                    _ => {}
                                }
                                sq.draw_square(&mut sout)?;
                                let a_win = winning_lines.test_for_a_win(&all_squares);
                                match a_win {
                                    None => {}
                                    Some(win_line) => {
                                        // So, the win_line we have here is a Vector of usize, with each being
                                        // a square in the all_squares list, I think. Let's see if we can print that out.
                                        for index_to_square_on_the_win_line in
                                            &win_line.square_indices
                                        {
                                            let this_square = all_squares
                                                .get(*index_to_square_on_the_win_line)
                                                .expect("202106252154");
                                            let index_to_these_coords = match win_line.direction {
                                                WinningLineDirections::Horizontal => {
                                                    this_square
                                                        .horizontal_winning_line_coord_indices
                                                }
                                                WinningLineDirections::Vertical => {
                                                    this_square.vertical_winning_line_coord_indices
                                                }
                                                WinningLineDirections::TopLeftBottomRight => {
                                                    this_square
                                                        .tlbr_diagonal_winning_line_coord_indices
                                                }
                                                WinningLineDirections::TopRightBottomLeft => {
                                                    this_square
                                                        .trbl_diagonal_winning_line_coord_indices
                                                }
                                            };
                                            let all_the_coords = winning_line_coords
                                                .return_a_set_of_coordinates(index_to_these_coords);
                                            for coord_in_the_square in all_the_coords.iter() {
                                                sout.queue(crossterm::cursor::MoveTo(
                                                    coord_in_the_square.x,
                                                    coord_in_the_square.y,
                                                ))?
                                                .queue(crossterm::style::Print("*"))?;
                                            }
                                        }
                                        sout.flush()?;
                                    }
                                }
                                players.next_player();
                            }
                        }
                    }
                    crossterm::event::KeyCode::Esc => {
                        keep_looping = false;
                    }
                    crossterm::event::KeyCode::Up => {
                        destination_square =
                            try_moving(square::Directions::North, &paths, cursor_square);
                    }
                    crossterm::event::KeyCode::Down => {
                        destination_square =
                            try_moving(square::Directions::South, &paths, cursor_square);
                    }
                    crossterm::event::KeyCode::Left => {
                        destination_square =
                            try_moving(square::Directions::West, &paths, cursor_square);
                    }
                    crossterm::event::KeyCode::Right => {
                        destination_square =
                            try_moving(square::Directions::East, &paths, cursor_square);
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        if destination_square.is_some() {
            let sq = all_squares
                .get(cursor_square)
                .expect("This really should find a square...");
            sq.draw_square(&mut sout)?; // will rub out current game cursor
            cursor_square = destination_square.unwrap();
            destination_square = None;
        }
    }

    exit_from_crossterm(&mut sout)?;

    Ok(())
}
