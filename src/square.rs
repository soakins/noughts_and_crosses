use std::io::Stdout;

use crossterm::Result;

use crate::square_contents::SquareContents;
use crate::square_size::SquareSize;

pub enum Directions {
    North,
    South,
    East,
    West,
}

pub struct PathsForCursor<'a> {
    pub from: &'a Square<'a>,
    pub to_north: Option<&'a Square<'a>>,
    pub to_south: Option<&'a Square<'a>>,
    pub to_east: Option<&'a Square<'a>>,
    pub to_west: Option<&'a Square<'a>>,
}

pub struct Square <'a> {
    pub contents: SquareContents,
    pub screen_x: u16,
    pub screen_y: u16,
    square_size: &'a SquareSize,
}
impl <'a> Square <'a> {
    pub fn new(screen_x: u16, screen_y: u16, square_size: &'a SquareSize) -> Self {
        Square {
            contents: SquareContents::Blank,
            screen_x,
            screen_y,
            square_size,
        }
    }
    pub fn set_contents(&mut self, c: SquareContents){
        self.contents = c;
    }
    pub fn spawn_another_square(&self, d: Directions) -> Square {
        
        let mut new_x = self.screen_x;
        let mut new_y = self.screen_y;

        match d {
            Directions::North => {
                new_y = self.screen_y - self.square_size.height - 1;
            },
            Directions::South => {
                new_y = self.screen_y + self.square_size.height + 1;
            },
            Directions::East => {
                new_x = self.screen_x + self.square_size.width + 1;
            },
            Directions::West => {
                new_x = self.screen_x - self.square_size.width - 1;
            },
        }

        Square::new (
            new_x,
            new_y,
            self.square_size
        )
    }

    pub fn draw_square(&self, sout: &mut Stdout) -> Result<()>{
        self.contents.draw_square_contents(sout, self.screen_x, self.screen_y, self.square_size)
    }
}
