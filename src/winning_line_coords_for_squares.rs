use crate::screen_coords::ScreenCoords;
pub struct WinningLineCoords {
    all_coords: Vec<ScreenCoords>,
}
impl WinningLineCoords {
    pub fn new() -> Self {
        WinningLineCoords {
            all_coords: Vec::new(),
        }
    }
    pub fn add_a_set_of_coordinates(
        &mut self,
        coord_list: &mut Vec<ScreenCoords>,
    ) -> (usize, usize) {
        if coord_list.len() == 0 {
            panic!("202106201455 Empty coordinate list provided.");
        }
        let first_index = self.all_coords.len();
        self.all_coords.append(coord_list);
        let last_index = self.all_coords.len();
        (first_index, last_index)
    }
    pub fn return_a_set_of_coordinates(&self, indices: (usize, usize)) -> &[ScreenCoords] {
        match indices {
            (first_index, last_index) => &self.all_coords[first_index..last_index],
        }
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
        set2.push(ScreenCoords { x: 6, y: 3 });
        set2.push(ScreenCoords { x: 100, y: 41 });
        set2.push(ScreenCoords { x: 8, y: 4 });
        wlc.add_a_set_of_coordinates(&mut set);
        let set2s_indices = wlc.add_a_set_of_coordinates(&mut set2);
        let result = wlc.return_a_set_of_coordinates(set2s_indices);

        panic!(
            "Result was {:?}, data returned was {:?}",
            set2s_indices, result
        );
    }
}
