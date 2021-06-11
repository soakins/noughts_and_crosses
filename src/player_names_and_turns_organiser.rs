use crate::square_contents::SquareContents;

pub struct PlayerDetails {
    pub player_name: String,
    pub player_mark: SquareContents,
}

pub struct PlayerNames {
    player1: PlayerDetails,
    player2: PlayerDetails,
    last_player_returned: Option<usize>, // last i.e. 'most recent'
    first_player_returned: Option<usize>,
    last_round_number: usize,
    round_number: usize,
}
impl PlayerNames {
    pub fn new(
        player1: String,
        player1_plays: SquareContents,
        player2: String,
        player2_plays: SquareContents,
    ) -> Self {
        PlayerNames {
            player1: PlayerDetails {
                player_name: player1,
                player_mark: player1_plays,
            },
            player2: PlayerDetails {
                player_name: player2,
                player_mark: player2_plays,
            },
            last_player_returned: None,
            first_player_returned: None,
            last_round_number: 0,
            round_number: 1,
        }
    }

    fn player_from_number(&self, n: Option<usize>) -> &PlayerDetails {
        match n {
            Some(0) => &self.player1,
            Some(1) => &self.player2,
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
        let pd = self.player_from_number(self.last_player_returned);
        format!(
            "Round {}: {} to play, using {}...",
            self.round_number, pd.player_name, pd.player_mark
        )
    }

    pub fn next_round(&mut self) {
        self.round_number += 1;
    }

    pub fn last_player(&self) -> &PlayerDetails {
        self.player_from_number(self.last_player_returned)
    }

    pub fn next_player(&mut self) {
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
    }
}
mod player_name_tests {
    use super::PlayerNames;
    use super::SquareContents;

    #[test]
    fn test_one() {
        let mut pn = PlayerNames::new(
            String::from("Sam"),
            SquareContents::X,
            String::from("Claire"),
            SquareContents::O,
        );
        for a in 1..5 {
            let pd = pn.last_player();
            println!(
                "  Attempt {}: {} playing as {}",
                a, pd.player_name, pd.player_mark
            );
        }
        pn.next_round();
        for b in 1..6 {
            let pd = pn.last_player();
            println!("  2nd round / Attempt {}: {}", b, pd.player_name);
        }
    }
}
