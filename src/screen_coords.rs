/// The ScreenCoords struct is intended for use with crossterm,
/// and so includes an x, y pair of u16 coordinate values.
#[derive(Clone, Copy)]
pub struct ScreenCoords {
    pub x: u16,
    pub y: u16,
}
