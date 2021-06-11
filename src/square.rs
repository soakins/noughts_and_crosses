use std::io::Stdout;

use crossterm::Result;

use crate::square_contents::SquareContents;
use crate::square_size::THE_SQUARE_SIZE;

#[derive(Debug, Clone, Copy)]
pub enum Directions {
    North,
    South,
    East,
    West,
}

pub struct PathsForCursor {
    pub from: usize,
    pub to_north: Option<usize>,
    pub to_south: Option<usize>,
    pub to_east: Option<usize>,
    pub to_west: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct Square {
    pub contents: SquareContents,
    pub screen_x: u16,
    pub screen_y: u16,
}
impl Square {
    pub fn new(screen_x: u16, screen_y: u16) -> Self {
        Square {
            contents: SquareContents::Blank,
            screen_x,
            screen_y,
        }
    }
    pub fn set_contents(&mut self, c: SquareContents) {
        self.contents = c;
    }
    pub fn get_contents(&self) -> SquareContents {
        self.contents
    }
    pub fn spawn_another_square(&self, d: Directions) -> Square {
        let mut new_x = self.screen_x;
        let mut new_y = self.screen_y;

        match d {
            Directions::North => {
                new_y = self.screen_y - THE_SQUARE_SIZE.height - 1;
            }
            Directions::South => {
                new_y = self.screen_y + THE_SQUARE_SIZE.height + 1;
            }
            Directions::East => {
                new_x = self.screen_x + THE_SQUARE_SIZE.width + 1;
            }
            Directions::West => {
                new_x = self.screen_x - THE_SQUARE_SIZE.width - 1;
            }
        }

        Square::new(new_x, new_y)
    }

    pub fn draw_square(&self, sout: &mut Stdout) -> Result<()> {
        self.contents
            .draw_square_contents(sout, self.screen_x, self.screen_y)
    }
}
