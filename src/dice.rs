use crate::ansi_draw::*;

pub const ONE_FACE: [&str; 5] = [
    " +-------+ ",
    "|         |",
    "|    #    |",
    "|         |",
    " +-------+ ",
];

pub const TWO_FACE: [&str; 5] = [
    " +-------+ ",
    "|       # |",
    "|         |",
    "| #       |",
    " +-------+ ",
];

pub const THREE_FACE: [&str; 5] = [
    " +-------+ ",
    "|       # |",
    "|    #    |",
    "| #       |",
    " +-------+ ",
];

pub const FOUR_FACE: [&str; 5] = [
    " +-------+ ",
    "| #     # |",
    "|         |",
    "| #     # |",
    " +-------+ ",
];

pub const FIVE_FACE: [&str; 5] = [
    " +-------+ ",
    "| #     # |",
    "|    #    |",
    "| #     # |",
    " +-------+ ",
];

pub const SIX_FACE: [&str; 5] = [
    " +-------+ ",
    "| #     # |",
    "| #     # |",
    "| #     # |",
    " +-------+ ",
];

pub fn draw_die_face(face: [&str; 5], point: (usize, usize)) {
    let (mut term_row, term_col) = point;
    for line in face {
        // Set cursor position
        let position = format!("\x1b[{};{}H", term_row, term_col);
        print!("\x1b[{};{}H{}{line}", term_row, term_col, position);
        // Move to next line for next write
        term_row += 1;
    }
}
