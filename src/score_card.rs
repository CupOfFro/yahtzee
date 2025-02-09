use crate::ansi_draw;

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

        ansi_draw::print_at((2, 3), "Name: ");

        ansi_draw::print_at((4, 3), "Ones: ");
        ansi_draw::print_at((5, 3), "Twos: ");
        ansi_draw::print_at((6, 3), "Thress: ");
        ansi_draw::print_at((7, 3), "Fours: ");
        ansi_draw::print_at((8, 3), "Fives: ");
        ansi_draw::print_at((9, 3), "Sixes: ");
        ansi_draw::print_at((10, 3), "Total of Upper: ");

        ansi_draw::print_at((12, 3), "3 of a kind: ");
        ansi_draw::print_at((13, 3), "4 of a kind: ");
        ansi_draw::print_at((14, 3), "Full House: ");
        ansi_draw::print_at((15, 3), "Sm Straight: ");
        ansi_draw::print_at((16, 3), "Lg Straight: ");
        ansi_draw::print_at((17, 3), "Yahtzee: ");
        ansi_draw::print_at((18, 3), "Chance: ");
        ansi_draw::print_at((19, 3), "Yahtzee Bonus: ");

        ansi_draw::print_at((21, 3), "Total of Lower:");
        ansi_draw::print_at((22, 3), "Total of Upper:");
        ansi_draw::print_at((23, 3), "Grand Total:");

        ansi_draw::draw_to_screen();
    }
}
