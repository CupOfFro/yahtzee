use crate::ansi_draw;
use crate::dice::*;

pub struct ScoreCard {
    name: String,
    // bool is if the section has been playedor not
    // usize is for score
    ones: (bool, usize),
    twos: (bool, usize),
    threes: (bool, usize),
    fours: (bool, usize),
    fives: (bool, usize),
    sixes: (bool, usize),
    // Lower section
    three_of_kind: (bool, usize),
    four_of_kind: (bool, usize),
    full_house: (bool, usize),
    sm_straight: (bool, usize),
    lg_straight: (bool, usize),
    yahtzee: (bool, usize),
    chance: (bool, usize),
    yahtzee_bonus: (bool, usize),
}

impl ScoreCard {
    pub fn new(name: &str) -> ScoreCard {
        ScoreCard {
            name: String::from(name),
            ones: (false, 0),
            twos: (false, 0),
            threes: (false, 0),
            fours: (false, 0),
            fives: (false, 0),
            sixes: (false, 0),
            // Lower section
            three_of_kind: (false, 0),
            four_of_kind: (false, 0),
            full_house: (false, 0),
            sm_straight: (false, 0),
            lg_straight: (false, 0),
            yahtzee: (false, 0),
            chance: (false, 0),
            yahtzee_bonus: (false, 0),
        }
    }

    pub fn draw(&self) {
        // Draw Borders
        ansi_draw::draw_horizontal_line((1, 2), "-", 56);
        ansi_draw::draw_horizontal_line((26, 2), "-", 56);
        ansi_draw::draw_vertical_line((1, 1), "|", 26);
        ansi_draw::draw_vertical_line((1, 57), "|", 26);

        // Still working on the ASCII functions here
        // so if you get a '12' then try to print a '2'
        // the card will have '22'. This probably won't
        // happen in a regular game since scores shouldn't go down
        // but to be safe (and for testing), I will print a few
        // spaces after the number, to erase anything
        let print_val = format!("Name: {}   ", self.name);
        ansi_draw::print_at((2, 3), &print_val);

        let mut sec_point: (usize, usize) = (4, 3);
        Self::format_score_at(&mut sec_point, self.ones, "Ones");
        Self::format_score_at(&mut sec_point, self.twos, "Twos");
        Self::format_score_at(&mut sec_point, self.threes, "Threes");
        Self::format_score_at(&mut sec_point, self.fours, "Fours");
        Self::format_score_at(&mut sec_point, self.fives, "Fives");
        Self::format_score_at(&mut sec_point, self.sixes, "Sixes");

        ansi_draw::print_at((10, 3), "Total of Upper: ");
        ansi_draw::print_at((11, 3), "Bonus (score >= 63): ");
        ansi_draw::print_at((12, 3), "Total of Upper: ");

        let mut sec_point: (usize, usize) = (14, 3);
        Self::format_score_at(&mut sec_point, self.three_of_kind, "3 of a kind");
        Self::format_score_at(&mut sec_point, self.four_of_kind, "4 of a kind");
        Self::format_score_at(&mut sec_point, self.full_house, "Full House");
        Self::format_score_at(&mut sec_point, self.sm_straight, "Sm Straight");
        Self::format_score_at(&mut sec_point, self.lg_straight, "Lg Straight");
        Self::format_score_at(&mut sec_point, self.yahtzee, "Yahtzee");
        Self::format_score_at(&mut sec_point, self.chance, "Chance");
        Self::format_score_at(&mut sec_point, self.yahtzee_bonus, "Yahtzee Bonus");

        ansi_draw::print_at((21, 3), "Total of Lower:");
        ansi_draw::print_at((22, 3), "Total of Upper:");
        ansi_draw::print_at((23, 3), "Grand Total:");

        ansi_draw::draw_to_screen();
    }

    fn format_score_at(pos: &mut (usize, usize), val: (bool, usize), val_str: &str) {
        // This formats a score if it is in use
        // It also increments the r,c point by one
        if val.0 {
            let print_val = format!("{}: {}   ", val_str, val.1);
            ansi_draw::print_at(*pos, &print_val);
        } else {
            let print_val = format!("{}:     ", val_str);
            ansi_draw::print_at(*pos, &print_val);
        }
        pos.0 += 1;
    }

    pub fn score_top(&mut self, category: usize, dice: &[Die; 5]) {
        let mut score = 0;
        for die in dice {
            if die.val == category {
                score += die.val
            }
        }
        match category {
            1 => {
                self.ones.0 = true;
                self.ones.1 = score;
            }
            2 => {
                self.twos.0 = true;
                self.twos.1 = score;
            }
            3 => {
                self.threes.0 = true;
                self.threes.1 = score;
            }
            4 => {
                self.fours.0 = true;
                self.fours.1 = score;
            }
            5 => {
                self.fives.0 = true;
                self.fives.1 = score;
            }
            6 => {
                self.sixes.0 = true;
                self.sixes.1 = score;
            }
            _ => (),
        }
    }

    pub fn score_3_of_kind(&mut self, dice: &[Die; 5]) {}
    pub fn score_4_of_kind(&mut self, dice: &[Die; 5]) {}
    pub fn score_full_house(&mut self, dice: &[Die; 5]) {}
    pub fn score_sm_straight(&mut self, dice: &[Die; 5]) {}
    pub fn score_lg_straight(&mut self, dice: &[Die; 5]) {}
    pub fn score_yahtzee(&mut self, dice: &[Die; 5]) {}
    pub fn score_chance(&mut self, dice: &[Die; 5]) {}
    pub fn score_yahtzee_bonus(&mut self, dice: &[Die; 5]) {}
}

#[cfg(test)]
mod dice_tests {
    use super::*;

    #[test]
    fn test_score_top_ones() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 1),
            Die::new((1, 1), 1),
            Die::new((1, 1), 1),
            Die::new((1, 1), 1),
            Die::new((1, 1), 1),
        ];
        score_card.score_top(1, &dice);
        assert_eq!(5, score_card.ones.1);
    }

    #[test]
    fn test_score_top_sixes() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
            Die::new((1, 1), 4),
            Die::new((1, 1), 6),
            Die::new((1, 1), 5),
        ];
        score_card.score_top(6, &dice);
        assert_eq!(18, score_card.sixes.1);
        assert_eq!(true, score_card.sixes.0);
    }
}
