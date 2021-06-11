use std::usize;

pub struct PlayerNames {
    pub player1: String,
    pub player2: String,
    last_player_returned: Option<usize>, // last i.e. 'most recent'
    first_player_returned: Option<usize>,
    last_round_number: usize,
    round_number: usize,
}
impl PlayerNames {
    pub fn new(player1: String, player2: String) -> Self {
        PlayerNames {
            player1,
            player2,
            last_player_returned: None,
            first_player_returned: None,
            last_round_number: 0,
            round_number: 1,
        }
    }

    fn player_name_from_number(&self, n: Option<usize>) -> &str {
        match n {
            Some(0) => self.player1.as_str(),
            Some(1) => self.player2.as_str(),
            None => {
                panic!("202106111153 None found as player name is sought.")
            }
            Some(x) => {
                panic!(
                    "202106111154 Number {} unexpected as player name is sought.",
                    x
                )
            }
        }
    }

    pub fn prompt_string(&self) -> String {
        format!(
            "Round {}: {} to play...",
            self.round_number,
            self.player_name_from_number(self.last_player_returned)
        )
    }

    pub fn next_round(&mut self) {
        self.round_number += 1;
    }

    pub fn next_player(&mut self) -> &str {
        if self.last_round_number < self.round_number {
            self.last_round_number = self.round_number;
            self.last_player_returned = None;
        }
        match self.last_player_returned {
            Some(last_player) => {
                let mut last_player_number = last_player + 1;
                if last_player_number > 1 {
                    last_player_number = 0;
                }
                self.last_player_returned = Some(last_player_number);
            }
            None => match self.first_player_returned {
                Some(first_player) => {
                    let mut first_player_number = first_player + 1;
                    if first_player_number > 1 {
                        first_player_number = 0;
                    }
                    self.first_player_returned = Some(first_player_number);
                    self.last_player_returned = Some(first_player_number);
                }
                None => {
                    self.first_player_returned = Some(0);
                    self.last_player_returned = Some(0);
                }
            },
        }
        self.player_name_from_number(self.last_player_returned)
    }
}
mod tests {
    use super::PlayerNames;

    #[test]
    fn test_one() {
        let mut pn = PlayerNames::new(String::from("Sam"), String::from("Claire"));
        for a in 1..5 {
            println!("  Attempt {}: {}", a, pn.next_player());
        }
        pn.next_round();
        for b in 1..6 {
            println!("  2nd round / Attempt {}: {}", b, pn.next_player());
        }
    }
}
