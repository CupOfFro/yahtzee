use crate::ansi_draw;
use crate::dice::*;

pub struct ScoreCard {
    name: String,

    ones: usize,
    twos: usize,
    threes: usize,
    fours: usize,
    fives: usize,
    sixes: usize,
    // Lower section
    three_of_kind: usize,
    four_of_kind: usize,
    full_house: usize,
    sm_straight: usize,
    lg_straight: usize,
    yahtzee: usize,
    chance: usize,
    yahtzee_bonus: usize,
}

impl ScoreCard {
    pub fn new(name: &str) -> ScoreCard {
        ScoreCard {
            name: String::from(name),
            ones: 0,
            twos: 0,
            threes: 0,
            fours: 0,
            fives: 0,
            sixes: 0,
            // Lower section
            three_of_kind: 0,
            four_of_kind: 0,
            full_house: 0,
            sm_straight: 0,
            lg_straight: 0,
            yahtzee: 0,
            chance: 0,
            yahtzee_bonus: 0,
        }
    }

    pub fn draw(&self) {
        // Draw Borders
        ansi_draw::draw_horizontal_line((1, 2), "-", 56);
        ansi_draw::draw_horizontal_line((24, 2), "-", 56);
        ansi_draw::draw_vertical_line((1, 1), "|", 24);
        ansi_draw::draw_vertical_line((1, 57), "|", 24);

        let print_val = format!("Name: {}", self.name);
        ansi_draw::print_at((2, 3), &print_val);

        let print_val = format!("Ones: {}", self.ones);
        ansi_draw::print_at((4, 3), &print_val);
        let print_val = format!("Twos: {}", self.twos);
        ansi_draw::print_at((5, 3), &print_val);
        let print_val = format!("Threes: {}", self.threes);
        ansi_draw::print_at((6, 3), &print_val);
        let print_val = format!("Fours: {}", self.fours);
        ansi_draw::print_at((7, 3), &print_val);
        let print_val = format!("Fives: {}", self.fives);
        ansi_draw::print_at((8, 3), &print_val);
        let print_val = format!("Sixes: {}", self.sixes);
        ansi_draw::print_at((9, 3), &print_val);

        ansi_draw::print_at((10, 3), "Total of Upper: ");

        let print_val = format!("3 of a kind: {}", self.three_of_kind);
        ansi_draw::print_at((12, 3), &print_val);
        let print_val = format!("4 of a kind: {}", self.four_of_kind);
        ansi_draw::print_at((13, 3), &print_val);
        let print_val = format!("Full House: {}", self.full_house);
        ansi_draw::print_at((14, 3), &print_val);
        let print_val = format!("Sm Straight: {}", self.sm_straight);
        ansi_draw::print_at((15, 3), &print_val);
        let print_val = format!("Lg Straight: {}", self.lg_straight);
        ansi_draw::print_at((16, 3), &print_val);
        let print_val = format!("Yahtzee: {}", self.yahtzee);
        ansi_draw::print_at((17, 3), &print_val);
        let print_val = format!("Chance: {}", self.chance);
        ansi_draw::print_at((18, 3), &print_val);
        let print_val = format!("Yahtzee Bonus: {}", self.yahtzee_bonus);
        ansi_draw::print_at((19, 3), &print_val);

        ansi_draw::print_at((21, 3), "Total of Lower:");
        ansi_draw::print_at((22, 3), "Total of Upper:");
        ansi_draw::print_at((23, 3), "Grand Total:");

        ansi_draw::draw_to_screen();
    }

    pub fn score_top(&mut self, category: usize, dice: &[Die; 5]) {
        let mut score = 0;
        for die in dice {
            if die.val == category {
                score += die.val
            }
        }
        match category {
            1 => self.ones = score,
            2 => self.twos = score,
            3 => self.threes = score,
            4 => self.fours = score,
            5 => self.fives = score,
            6 => self.sixes = score,
            _ => (),
        }
    }
}

#[cfg(test)]
mod dice_tests {
    use super::*;

    #[test]
    fn test_score_top_ones(){
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1,1),1),
            Die::new((1,1),1),
            Die::new((1,1),1),
            Die::new((1,1),1),
            Die::new((1,1),1),
        ];
        score_card.score_top(1, &dice);
        assert_eq!(5, score_card.ones );
    }

    #[test]
    fn test_score_top_sixes(){
        let mut score_card = ScoreCard::new("Test");
        let dice = [
            Die::new((1,1),6),
            Die::new((1,1),6),
            Die::new((1,1),4),
            Die::new((1,1),6),
            Die::new((1,1),5),
        ];
        score_card.score_top(6, &dice);
        assert_eq!(18, score_card.sixes );
    }
}