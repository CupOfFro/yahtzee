use crate::ansi_draw;
use rand::Rng;

const ONE_FACE: [&str; 5] = [
    " +-------+ ",
    "|         |",
    "|    #    |",
    "|         |",
    " +-------+ ",
];

const TWO_FACE: [&str; 5] = [
    " +-------+ ",
    "|       # |",
    "|         |",
    "| #       |",
    " +-------+ ",
];

const THREE_FACE: [&str; 5] = [
    " +-------+ ",
    "|       # |",
    "|    #    |",
    "| #       |",
    " +-------+ ",
];

const FOUR_FACE: [&str; 5] = [
    " +-------+ ",
    "| #     # |",
    "|         |",
    "| #     # |",
    " +-------+ ",
];

const FIVE_FACE: [&str; 5] = [
    " +-------+ ",
    "| #     # |",
    "|    #    |",
    "| #     # |",
    " +-------+ ",
];

const SIX_FACE: [&str; 5] = [
    " +-------+ ",
    "| #     # |",
    "| #     # |",
    "| #     # |",
    " +-------+ ",
];

// Add structure here
// make faces private
pub struct Die {
    row: usize,
    col: usize,
    pub val: usize,
    rollable: bool, // If true we will roll, else we won't
}

impl Die {
    pub fn new(pos: (usize, usize), val: usize) -> Die {
        Die {
            row: pos.0,
            col: pos.1,
            val: if val < 1 || val > 6 {
                rand::thread_rng().gen_range(1..=6)
            } else {
                val
            },
            rollable: true,
        }
    }

    pub fn draw(&self, highlighted: bool) {
        let face = match self.val {
            1 => ONE_FACE,
            2 => TWO_FACE,
            3 => THREE_FACE,
            4 => FOUR_FACE,
            5 => FIVE_FACE,
            6 => SIX_FACE,
            _ => ONE_FACE,
        };

        if highlighted {
            ansi_draw::set_bg_color(ansi_draw::ANSI_BLUE_BG);
        }
        draw_die_face(face, (self.row, self.col));
        ansi_draw::set_bg_color(ansi_draw::ANSI_RESET_TEXT);
    }

    pub fn roll(&mut self) {
        if self.rollable {
            self.val = rand::thread_rng().gen_range(1..=6);
        }
    }
}

fn draw_die_face(face: [&str; 5], point: (usize, usize)) {
    let (mut term_row, term_col) = point;
    for line in face {
        // Set cursor position
        let position = format!("\x1b[{};{}H", term_row, term_col);
        print!("\x1b[{};{}H{}{line}", term_row, term_col, position);
        // Move to next line for next write
        term_row += 1;
    }
}
