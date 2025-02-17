use crate::ansi_draw;
use crate::dice::*;

const ONES: usize = 1;
const TWOS: usize = 2;
const THREES: usize = 3;
const FOURS: usize = 4;
const FIVES: usize = 5;
const SIXES: usize = 6;
const THREE_OF_KIND: usize = 7;
const FOUR_OF_KIND: usize = 8;
const FULL_HOUSE: usize = 9;
const SMALL_STRAIGHT: usize = 10;
const LARGE_STRAIGHT: usize = 11;
const YAHTZEE: usize = 12;
const CHANCE: usize = 13;
const YAHTZEE_BONUS: usize = 14;

pub struct ScoreCard {
    name: String,
    // bool is if the section has been played or not
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

    pub selection: usize,
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

            selection: 1,
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
        Self::format_score_at(&mut sec_point, self.ones, "Ones", self.selection, ONES);
        Self::format_score_at(&mut sec_point, self.twos, "Twos", self.selection, TWOS);
        Self::format_score_at(
            &mut sec_point,
            self.threes,
            "Threes",
            self.selection,
            THREES,
        );
        Self::format_score_at(&mut sec_point, self.fours, "Fours", self.selection, FOURS);
        Self::format_score_at(&mut sec_point, self.fives, "Fives", self.selection, FIVES);
        Self::format_score_at(&mut sec_point, self.sixes, "Sixes", self.selection, SIXES);

        ansi_draw::set_bg_color(ansi_draw::ANSI_RESET_TEXT);
        ansi_draw::print_at((10, 3), "Total of Upper: ");
        ansi_draw::print_at((11, 3), "Bonus (score >= 63): ");
        ansi_draw::print_at((12, 3), "Total of Upper: ");

        let mut sec_point: (usize, usize) = (14, 3);
        Self::format_score_at(
            &mut sec_point,
            self.three_of_kind,
            "3 of a kind",
            self.selection,
            THREE_OF_KIND,
        );
        Self::format_score_at(
            &mut sec_point,
            self.four_of_kind,
            "4 of a kind",
            self.selection,
            FOUR_OF_KIND,
        );
        Self::format_score_at(
            &mut sec_point,
            self.full_house,
            "Full House",
            self.selection,
            FULL_HOUSE,
        );
        Self::format_score_at(
            &mut sec_point,
            self.sm_straight,
            "Sm Straight",
            self.selection,
            SMALL_STRAIGHT,
        );
        Self::format_score_at(
            &mut sec_point,
            self.lg_straight,
            "Lg Straight",
            self.selection,
            LARGE_STRAIGHT,
        );
        Self::format_score_at(
            &mut sec_point,
            self.yahtzee,
            "Yahtzee",
            self.selection,
            YAHTZEE,
        );
        Self::format_score_at(
            &mut sec_point,
            self.chance,
            "Chance",
            self.selection,
            CHANCE,
        );
        Self::format_score_at(
            &mut sec_point,
            self.yahtzee_bonus,
            "Yahtzee Bonus",
            self.selection,
            YAHTZEE_BONUS,
        );

        ansi_draw::set_bg_color(ansi_draw::ANSI_RESET_TEXT);
        ansi_draw::print_at((22, 3), "Total of Lower:");
        ansi_draw::print_at((23, 3), "Total of Upper:");
        ansi_draw::print_at((24, 3), "Grand Total:");

        ansi_draw::draw_to_screen();
    }

    fn format_score_at(
        pos: &mut (usize, usize),
        val: (bool, usize),
        val_str: &str,
        selection: usize,
        target: usize,
    ) {
        // This formats a score if it is in use
        // It also increments the r,c point by one
        if selection == target {
            ansi_draw::set_bg_color(ansi_draw::ANSI_WHITE_BG);
        } else if val.0 == true {
            ansi_draw::set_bg_color(ansi_draw::ANSI_BLUE_BG);
        } else {
            ansi_draw::set_bg_color(ansi_draw::ANSI_RESET_TEXT);
        }

        if val.0 {
            let print_val = format!("{}: {}{}   ", val_str, val.1, ansi_draw::ANSI_RESET_TEXT);
            ansi_draw::print_at(*pos, &print_val);
        } else {
            let print_val = format!("{}:     ", val_str);
            ansi_draw::print_at(*pos, &print_val);
        }
        pos.0 += 1;
    }

    // Return true if we set a value, false otherwise
    pub fn score(&mut self, dice: &[Die; 5]) -> bool {
        if self.selection == 1 {
            if self.ones.0 == false {
                self.score_top(1, dice);
                return true;
            }
        } else if self.selection == 2 {
            if self.twos.0 == false {
                self.score_top(2, dice);
                return true;
            }
        } else if self.selection == 3 {
            if self.threes.0 == false {
                self.score_top(3, dice);
                return true;
            }
        } else if self.selection == 4 {
            if self.fours.0 == false {
                self.score_top(4, dice);
                return true;
            }
        } else if self.selection == 5 {
            if self.fives.0 == false {
                self.score_top(5, dice);
                return true;
            }
        } else if self.selection == 6 {
            if self.sixes.0 == false {
                self.score_top(6, dice);
                return true;
            }
        }

        false
    }

    fn score_top(&mut self, category: usize, dice: &[Die; 5]) {
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

    fn score_3_of_kind(&mut self, dice: &[Die; 5]) {
        self.three_of_kind.0 = true;
        // We are using 1-6 so we need 7 elements
        let mut num_of_die = [0; 7];
        // Also add up score in case we are 3 of a kind
        let mut score = 0;
        for die in dice {
            num_of_die[die.val] += 1;
            score += die.val;
        }

        for val in num_of_die {
            if val >= 3 {
                self.three_of_kind.1 = score;
                return;
            }
        }

        self.three_of_kind.1 = 0;
    }

    fn score_4_of_kind(&mut self, dice: &[Die; 5]) {
        self.four_of_kind.0 = true;
        // We are using 1-6 so we need 7 elements
        let mut num_of_die = [0; 7];
        // Also add up score in case we are 3 of a kind
        let mut score = 0;
        for die in dice {
            num_of_die[die.val] += 1;
            score += die.val;
        }

        for val in num_of_die {
            if val >= 4 {
                self.four_of_kind.1 = score;
                return;
            }
        }

        self.four_of_kind.1 = 0;
    }

    fn score_full_house(&mut self, dice: &[Die; 5]) {
        self.full_house.0 = true;
        // We are using 1-6 so we need 7 elements
        let mut num_of_die = [0; 7];
        for die in dice {
            num_of_die[die.val] += 1;
        }

        let mut found_two = false;
        let mut found_three = false;

        for val in num_of_die {
            if val == 2 {
                found_two = true;
            } else if val == 3 {
                found_three = true;
            }
        }
        if found_two && found_three {
            self.full_house.1 = 25;
        } else {
            self.full_house.1 = 0;
        }
    }

    fn score_sm_straight(&mut self, dice: &[Die; 5]) {
        // We have a small straight with 1-4 or 2-5 or 3-6
        self.sm_straight.0 = true;
        let mut die_arr: [usize; 5] = [0; 5];
        let mut i = 0;
        for die in dice {
            die_arr[i] = die.val;
            i += 1;
        }
        die_arr.sort();
        // We don't care what the fifth die is but we need the arrays to
        // match for the small straight portion
        let valid_sm_straight_one = [1, 2, 3, 4, die_arr[4]];
        let valid_sm_straight_two = [2, 3, 4, 5, die_arr[4]];
        let valid_sm_straight_three = [2, 3, 4, 5, die_arr[4]];
        if die_arr == valid_sm_straight_one
            || die_arr == valid_sm_straight_two
            || die_arr == valid_sm_straight_three
        {
            self.sm_straight.1 = 30;
        } else {
            self.sm_straight.1 = 0;
        }
    }

    fn score_lg_straight(&mut self, dice: &[Die; 5]) {
        // We have a large straight with 1-5 or 2-6
        self.lg_straight.0 = true;
        let mut die_arr: [usize; 5] = [0; 5];
        let mut i = 0;
        for die in dice {
            die_arr[i] = die.val;
            i += 1;
        }
        die_arr.sort();
        let valid_lg_straight_one = [1, 2, 3, 4, 5];
        let valid_lg_straight_two = [2, 3, 4, 5, 6];
        if die_arr == valid_lg_straight_one || die_arr == valid_lg_straight_two {
            self.lg_straight.1 = 40;
        } else {
            self.lg_straight.1 = 0;
        }
    }

    fn score_yahtzee(&mut self, dice: &[Die; 5]) {
        self.yahtzee.0 = true;
        if dice[0].val == dice[1].val
            && dice[0].val == dice[2].val
            && dice[0].val == dice[3].val
            && dice[0].val == dice[4].val
        {
            self.yahtzee.1 = 50;
        }
    }

    fn score_chance(&mut self, dice: &[Die; 5]) {
        let mut score = 0;
        for die in dice {
            score += die.val;
        }
        self.chance = (true, score);
    }

    fn score_yahtzee_bonus(&mut self, dice: &[Die; 5]) {
        if self.yahtzee.0 == true && self.yahtzee.1 != 0 {
            self.yahtzee_bonus.0 = true;
            self.yahtzee_bonus.1 += 100;
        }
    }
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
        assert_eq!((true, 5), score_card.ones);
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
        assert_eq!((true, 18), score_card.sixes);
    }

    #[test]
    fn test_score_got_yahtzee() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
        ];
        score_card.score_yahtzee(&dice);
        assert_eq!((true, 50), score_card.yahtzee);
    }

    #[test]
    fn test_score_not_yahtzee() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
            Die::new((1, 1), 3),
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
        ];
        score_card.score_yahtzee(&dice);
        assert_eq!((true, 0), score_card.yahtzee);
    }

    #[test]
    fn test_score_chance() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 5),
            Die::new((1, 1), 2),
            Die::new((1, 1), 4),
            Die::new((1, 1), 6),
            Die::new((1, 1), 3),
        ];
        score_card.score_chance(&dice);
        assert_eq!((true, 20), score_card.chance);
    }

    #[test]
    fn test_score_yahtzee_bonus() {
        let mut score_card = ScoreCard::new("Test");
        score_card.yahtzee = (true, 50);
        let dice = [
            Die::new((1, 1), 5),
            Die::new((1, 1), 5),
            Die::new((1, 1), 5),
            Die::new((1, 1), 5),
            Die::new((1, 1), 5),
        ];
        score_card.score_yahtzee_bonus(&dice);
        assert_eq!((true, 100), score_card.yahtzee_bonus);
        score_card.score_yahtzee_bonus(&dice);
        assert_eq!((true, 200), score_card.yahtzee_bonus);
        score_card.score_yahtzee_bonus(&dice);
        assert_eq!((true, 300), score_card.yahtzee_bonus);
    }

    #[test]
    fn test_score_lg_straight_pass() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 4),
            Die::new((1, 1), 3),
            Die::new((1, 1), 1),
            Die::new((1, 1), 2),
            Die::new((1, 1), 5),
        ];
        score_card.score_lg_straight(&dice);
        assert_eq!((true, 40), score_card.lg_straight);
    }

    #[test]
    fn test_score_lg_straight_fail() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 4),
            Die::new((1, 1), 3),
            Die::new((1, 1), 1),
            Die::new((1, 1), 2),
            Die::new((1, 1), 6),
        ];
        score_card.score_lg_straight(&dice);
        assert_eq!((true, 0), score_card.lg_straight);
    }

    #[test]
    fn test_score_sm_straight_pass() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 4),
            Die::new((1, 1), 3),
            Die::new((1, 1), 1),
            Die::new((1, 1), 2),
            Die::new((1, 1), 6),
        ];
        score_card.score_sm_straight(&dice);
        assert_eq!((true, 30), score_card.sm_straight);
    }

    #[test]
    fn test_score_sm_straight_fail() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 3),
            Die::new((1, 1), 1),
            Die::new((1, 1), 2),
            Die::new((1, 1), 6),
        ];
        score_card.score_sm_straight(&dice);
        assert_eq!((true, 0), score_card.sm_straight);
    }

    #[test]
    fn test_score_3_of_kind() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 2),
            Die::new((1, 1), 2),
            Die::new((1, 1), 2),
            Die::new((1, 1), 6),
        ];
        score_card.score_3_of_kind(&dice);
        assert_eq!((true, 18), score_card.three_of_kind);
    }

    #[test]
    fn test_score_3_of_kind_zero() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 3),
            Die::new((1, 1), 2),
            Die::new((1, 1), 2),
            Die::new((1, 1), 6),
        ];
        score_card.score_3_of_kind(&dice);
        assert_eq!((true, 0), score_card.three_of_kind);
    }

    #[test]
    fn test_score_4_of_kind() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 5),
            Die::new((1, 1), 5),
            Die::new((1, 1), 5),
            Die::new((1, 1), 5),
        ];
        score_card.score_4_of_kind(&dice);
        assert_eq!((true, 26), score_card.four_of_kind);
    }

    #[test]
    fn test_score_4_of_kind_zero() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 2),
            Die::new((1, 1), 3),
            Die::new((1, 1), 3),
            Die::new((1, 1), 3),
        ];
        score_card.score_4_of_kind(&dice);
        assert_eq!((true, 0), score_card.four_of_kind);
    }

    #[test]
    fn test_score_full_house() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 6),
            Die::new((1, 1), 3),
            Die::new((1, 1), 3),
            Die::new((1, 1), 3),
        ];
        score_card.score_full_house(&dice);
        assert_eq!((true, 25), score_card.full_house);
    }
    #[test]
    fn test_score_full_house_zero() {
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1, 1), 6),
            Die::new((1, 1), 3),
            Die::new((1, 1), 3),
            Die::new((1, 1), 3),
            Die::new((1, 1), 3),
        ];
        score_card.score_full_house(&dice);
        assert_eq!((true, 0), score_card.full_house);
    }
}
