use crate::{square::Square, square_contents::SquareContents};
pub struct WinningLines(Vec<Vec<usize>>);
impl WinningLines {
    pub fn new() -> Self {
        WinningLines(Vec::new())
    }
    pub fn add_a_line(&mut self, first_square: usize, second_square: usize, third_square: usize) {
        let this_line: Vec<usize> = vec![first_square, second_square, third_square];
        self.0.push(this_line);
    }

    pub fn test_for_a_win(&self, all_squares: &Vec<Square>) -> Option<&Vec<usize>> {
        let mut retvar: Option<&Vec<usize>> = None;
        for this_line in self.0.iter() {
            retvar = Some(this_line);
            let mut selected_square_content: Option<SquareContents> = None; // we'll remember the content here, to match with the other squares in the line
            for this_line_square in this_line.iter() {
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
