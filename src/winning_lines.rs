struct WinningLines(Vec<Vec<usize>>);
impl WinningLines {
    fn new() -> Self {
        WinningLines(Vec::new())
    }
    fn add_a_line(&mut self, first_square: usize, second_square: usize, third_square: usize) {
        let this_line: Vec<usize> = vec![first_square, second_square, third_square];
        more here
    }
}
