use std::io::Stdout;

use crossterm::Result;

use crate::square_contents::SquareContents;

pub struct Square {
    pub contents: SquareContents,
    screen_x: u16,
    screen_y: u16,
    width: u16,
    height: u16,
}
impl Square {
    pub fn new(contents: SquareContents, screen_x: u16, screen_y: u16, width: u16, height: u16) -> Self {
        Square {
            contents,
            screen_x,
            width,
            screen_y,
            height,
        }
    }
    pub fn draw_square(&self, sout: &mut Stdout) -> Result<()>{
        self.contents.draw_square_contents(sout, self.screen_x, self.screen_y, self.width, self.height)
    }
}
