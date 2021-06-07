pub enum AllowableSizes {
    Size5x5,
    Size8x8,
}

pub struct SquareSize{
    pub stated_size: AllowableSizes,
    pub width: u16,
    pub height: u16,
}
impl SquareSize {
    pub fn from_allowable_size(size: AllowableSizes) -> Self {
        match size {
            AllowableSizes::Size5x5 => SquareSize{stated_size: size, width: 5, height: 5,},
            AllowableSizes::Size8x8 => SquareSize{stated_size: size, width: 8, height: 8,},
        }
    }
}