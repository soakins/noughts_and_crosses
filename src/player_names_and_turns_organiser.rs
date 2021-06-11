struct PlayerNames {
    player1: String,
    player2: String,
}
impl PlayerNames {
    fn new(player1: String, player2: String) -> Self {
        PlayerNames {player1, player2}
    }

    fn player_loop_object(&self, round_number: u16) -> Vec<&str> {

        let mut retvar: Vec<&str> = Vec::new();

        if round_number % 2 == 0 {

            retvar.push(self.player1.as_str());
            retvar.push(self.player2.as_str());

        }
        else {

            retvar.push(self.player2.as_str());
            retvar.push(self.player1.as_str());

        }

        retvar
    }
}
mod tests {

    use super::PlayerNames;

    #[test]
    fn test_one(){
        let pn = PlayerNames::new(String::from("cat"), String::from("dog"));
        for i in 1..10 {
            print!("{}", i);
            for a in pn.player_loop_object(i) {
                println!("{}", a);
            }
        }
    }
}