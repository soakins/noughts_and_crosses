use crate::{CHARACTER_H, CHARACTER_V};

/*
pub const THE_SQUARE_SIZE: SquareSize = SquareSize {
    width: 9,
    height: 9,
};
*/

pub const THE_SQUARE_SIZE: SquareSize = SquareSize::new(9, 9);

pub struct SquareSize {
    pub width: u16,
    pub height: u16,
    pub blank_square_lines: Vec<String>,
    pub x_square_lines: Vec<String>,
    pub o_square_lines: Vec<String>,
}
impl SquareSize {
    pub fn new(width: u16, height: u16) -> Self {
        let x_lines: Vec<String> = Vec::new();
        let o_lines: Vec<String> = Vec::new();
        let blank_lines: Vec<String> = Vec::new();

        match (width, height) {
            (9, 9) => {
                let line = " ".repeat(8).push(CHARACTER_V);
                x_lines.push(line);
                let line = x_lines.push(" *    * ");
                x_lines.push("  *  *  ");
                x_lines.push("   **   ");
                x_lines.push("   **   ");
                x_lines.push("  *  *  ");
                x_lines.push(" *    * ");
                x_lines.push("        ");
            }
        }

        SquareSize {
            width,
            height,
            blank_square_lines: Vec::new(),
            x_square_lines: Vec::new(),
            o_square_lines: Vec::new(),
        }
    }
}
