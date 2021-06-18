use crate::screen_coords::ScreenCoords;
pub struct WinningLineCoords(Vec<ScreenCoords>);
impl WinningLineCoords {
    fn new() -> Self {
        WinningLineCoords(Vec::new())
    }
    fn add_a_set_of_coordinates(&mut self, coord_list: &mut Vec<ScreenCoords>) -> (usize, usize) {
        this won't work if the incoming Vec is empty. what should it do. And what type should be returned? a slice? or two usizes for searching the long vec.
        let first_index = self.0.len();
        self.0.append(coord_list);
        let last_index = self.0.len() - 1;
        (first_index, last_index)
    }
}

mod tests {
    use super::ScreenCoords;
    use super::WinningLineCoords;
    #[test]
    fn test_for_adding_a_set_of_coordinates() {
        let mut wlc = WinningLineCoords::new();
        let mut set: Vec<ScreenCoords> = Vec::new();
        set.push(ScreenCoords { x: 1, y: 1 });
        set.push(ScreenCoords { x: 2, y: 2 });
        let mut set2: Vec<ScreenCoords> = Vec::new();
        set2.push(ScreenCoords { x: 1, y: 1 });
        set2.push(ScreenCoords { x: 2, y: 2 });
        wlc.add_a_set_of_coordinates(&mut set);
        let result = wlc.add_a_set_of_coordinates(&mut set2);
        panic!("Result was {:?}", result);
    }
}
