use std::fmt::{write, Display};

use crate::{square::Square, square_contents::SquareContents};

#[derive(Clone, Copy)]
pub enum WinningLineDirections {
    Vertical,
    Horizontal,
    Top_Left_Bottom_Right,
    Top_Right_Bottom_Left,
}
impl Display for WinningLineDirections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Self::Vertical => "Vertical",
            Self::Horizontal => "Horizontal",
            Self::Top_Left_Bottom_Right => "TLBR",
            Self::Top_Right_Bottom_Left => "TRBL",
        };
        write(f, format_args!("{}", out))
    }
}

pub struct OneWinningLine {
    pub square_indices: Vec<usize>,
    pub direction: WinningLineDirections,
}
pub struct WinningLines(Vec<OneWinningLine>);
impl WinningLines {
    pub fn new() -> Self {
        WinningLines(Vec::new())
    }
    pub fn add_a_line(
        &mut self,
        first_square: usize,
        second_square: usize,
        third_square: usize,
        direction: WinningLineDirections,
    ) {
        let this_line = OneWinningLine {
            square_indices: vec![first_square, second_square, third_square],
            direction,
        };
        self.0.push(this_line);
    }

    pub fn test_for_a_win(&self, all_squares: &Vec<Square>) -> Option<&OneWinningLine> {
        let mut retvar: Option<&OneWinningLine> = None;
        for this_line in self.0.iter() {
            retvar = Some(this_line);
            let mut selected_square_content: Option<SquareContents> = None; // we'll remember the content here, to match with the other squares in the line
            for this_line_square in this_line.square_indices.iter() {
                let this_line_square_content = all_squares
                    .get(this_line_square.clone())
                    .unwrap()
                    .get_contents();
                match this_line_square_content {
                    SquareContents::Blank => {
                        retvar = None;
                        break;
                    }
                    _ => match selected_square_content {
                        None => selected_square_content = Some(this_line_square_content),
                        Some(ssc) => {
                            if this_line_square_content != ssc {
                                retvar = None;
                                break;
                            }
                        }
                    },
                }
            }
            if retvar.is_some() {
                break;
            }
        }
        retvar
    }
}
