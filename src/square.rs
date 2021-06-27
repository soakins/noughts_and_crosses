use std::io::Stdout;

use crossterm::Result;
use winning_line_coords_for_squares::WinningLineCoords;

use crate::screen_coords::ScreenCoords;
use crate::square_contents::SquareContents;
use crate::square_size::THE_SQUARE_SIZE;
use crate::winning_line_coords_for_squares;

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
    pub screen_coords: ScreenCoords,
    pub vertical_winning_line_coord_indices: (usize, usize),
    pub horizontal_winning_line_coord_indices: (usize, usize),
    pub tlbr_diagonal_winning_line_coord_indices: (usize, usize),
    pub trbl_diagonal_winning_line_coord_indices: (usize, usize),
}
impl Square {
    pub fn new(screen_x: u16, screen_y: u16, winning_line_coords: &mut WinningLineCoords) -> Self {
        let c = ScreenCoords {
            x: screen_x,
            y: screen_y,
        };
        let mut vert_w_line_coords: Vec<ScreenCoords> = Vec::new();
        let wlx = screen_x + (THE_SQUARE_SIZE.width / 2); // having done a little test, this division will round the result down if the denominator is an odd number...
        for wly in screen_y..screen_y + THE_SQUARE_SIZE.height {
            let sc = ScreenCoords { x: wlx, y: wly };
            vert_w_line_coords.push(sc);
        }
        let vert_w_line_coord_indices =
            winning_line_coords.add_a_set_of_coordinates(&mut vert_w_line_coords);

        let mut horiz_w_line_coords: Vec<ScreenCoords> = Vec::new();
        let wly = screen_y + (THE_SQUARE_SIZE.height / 2);
        for wlx in screen_x..screen_x + THE_SQUARE_SIZE.width {
            let sc = ScreenCoords { x: wlx, y: wly };
            horiz_w_line_coords.push(sc);
        }
        let horiz_w_line_coord_indices =
            winning_line_coords.add_a_set_of_coordinates(&mut horiz_w_line_coords);

        let mut tlbr_w_line_coords: Vec<ScreenCoords> = Vec::new();
        let mut trbl_w_line_coords: Vec<ScreenCoords> = Vec::new();
        let mut tlbr_wlx_low = screen_x;
        let mut tlbr_wlx_high = screen_x + THE_SQUARE_SIZE.width;
        let mut tlbr_wly_low = screen_y;
        let mut tlbr_wly_high = screen_y + THE_SQUARE_SIZE.height;
        let mut trbl_wlx_low = screen_x;
        let mut trbl_wlx_high = screen_x + THE_SQUARE_SIZE.width;
        let mut trbl_wly_low = screen_y;
        let mut trbl_wly_high = screen_y + THE_SQUARE_SIZE.height;

        loop {
            tlbr_w_line_coords.push(ScreenCoords {
                x: tlbr_wlx_low,
                y: tlbr_wly_low,
            });
            tlbr_w_line_coords.push(ScreenCoords {
                x: tlbr_wlx_high,
                y: tlbr_wly_high,
            });
            trbl_w_line_coords.push(ScreenCoords {
                x: trbl_wlx_high,
                y: trbl_wly_low,
            });
            trbl_w_line_coords.push(ScreenCoords {
                x: trbl_wlx_low,
                y: trbl_wly_high,
            });
            tlbr_wlx_low += 1u16;
            tlbr_wlx_high -= 1u16;
            tlbr_wly_low += 1u16;
            tlbr_wly_high -= 1u16;
            trbl_wlx_low += 1u16;
            trbl_wlx_high -= 1u16;
            trbl_wly_low += 1u16;
            trbl_wly_high -= 1u16;
            if tlbr_wlx_high < tlbr_wlx_low || tlbr_wly_high < tlbr_wly_low {
                break;
            }
        }

        let tlbr_w_line_coord_indices =
            winning_line_coords.add_a_set_of_coordinates(&mut tlbr_w_line_coords);
        let trbl_w_line_coord_indices =
            winning_line_coords.add_a_set_of_coordinates(&mut trbl_w_line_coords);

        Square {
            contents: SquareContents::Blank,
            screen_coords: c,
            vertical_winning_line_coord_indices: vert_w_line_coord_indices,
            horizontal_winning_line_coord_indices: horiz_w_line_coord_indices,
            tlbr_diagonal_winning_line_coord_indices: tlbr_w_line_coord_indices,
            trbl_diagonal_winning_line_coord_indices: trbl_w_line_coord_indices,
        }
    }
    pub fn set_contents(&mut self, c: SquareContents) {
        self.contents = c;
    }
    pub fn get_contents(&self) -> SquareContents {
        self.contents
    }
    pub fn spawn_another_square(
        &self,
        d: Directions,
        winning_line_coords: &mut WinningLineCoords,
    ) -> Square {
        let mut new_x = self.screen_coords.x;
        let mut new_y = self.screen_coords.y;

        match d {
            Directions::North => {
                new_y = self.screen_coords.y - THE_SQUARE_SIZE.height - 1;
            }
            Directions::South => {
                new_y = self.screen_coords.y + THE_SQUARE_SIZE.height + 1;
            }
            Directions::East => {
                new_x = self.screen_coords.x + THE_SQUARE_SIZE.width + 1;
            }
            Directions::West => {
                new_x = self.screen_coords.x - THE_SQUARE_SIZE.width - 1;
            }
        }

        Square::new(new_x, new_y, winning_line_coords)
    }

    pub fn draw_square(&self, sout: &mut Stdout) -> Result<()> {
        self.contents
            .draw_square_contents(sout, self.screen_coords.x, self.screen_coords.y)
    }
}
