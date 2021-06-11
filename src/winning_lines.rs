use crate::{square::Square, square_contents::SquareContents};
struct WinningLines(Vec<Vec<usize>>);
impl WinningLines {
    fn new() -> Self {
        WinningLines(Vec::new())
    }
    fn add_a_line(&mut self, first_square: usize, second_square: usize, third_square: usize) {
        let this_line: Vec<usize> = vec![first_square, second_square, third_square];
        self.0.push(this_line);
    }

    fn test_for_a_win(&self, all_squares: &Vec<Square>) -> Option<&Vec<usize>> {
        for this_line in self.0.iter() {
            let mut selected_square_content: Option<SquareContents> = None;   // we'll remember the content here, to match with the other squares in the line
            for this_line_square in this_line.iter() {
                let this_line_square_content = all_squares.get(this_line_square.clone()).unwrap().get_contents();
                match this_line_square_content {
                    SquareContents::Blank => {}
                }
                match selected_square_content {
                    None => selected_square_content = Some(this_line_square_content),
                    Some(c) => {
                        
                    }
                }
            }
        }
    }
}
