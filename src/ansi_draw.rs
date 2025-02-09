use std::io;
use std::io::Write;

// usefule references for ANSI escape codes
// https://stackoverflow.com/questions/33139248/i-cannot-print-color-escape-codes-to-the-terminal
// https://stackoverflow.com/questions/69597466/move-cursor-escape-sequence
// https://duffney.io/usingansiescapesequencespowershell/
// https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
// https://en.wikipedia.org/wiki/ANSI_escape_code

// Info about format args for rust
// https://doc.rust-lang.org/std/fmt/index.html

pub fn set_bg_color(color: &str) {
    print!("{}", color);
}

// This will draw any print statements we have done with no new lines
pub fn draw_to_screen() {
    io::stdout().flush().expect("Welp this is bad");
}

pub fn draw_horizontal_line(point: (usize, usize), char: &str, length: usize) {
    // I could use format to do this but think it looks better to use ANSI escapes like most of program
    let (term_row, mut term_col) = point;
    for i in 0..length {
        let position = format!("\x1b[{};{}H", term_row, term_col);
        term_col += 1;
        print!("{}{}", position, char);
    }
}

pub fn draw_vertical_line(point: (usize, usize), char: &str, length: usize) {
    let (mut term_row, term_col) = point;
    for i in 0..length {
        let position = format!("\x1b[{};{}H", term_row, term_col);
        term_row += 1;
        print!("{}{}", position, char);
    }
}

pub const ANSI_CLEAR_SCREEN: &str = "\x1b[2J";
pub const ANSI_HOME: &str = "\x1b[H";
pub const ANSI_RESET_TEXT: &str = "\x1b[0m";

// Colors
// pub const ANSI_BLACK_TEXT: &str = "\x1b[30m";
// pub const ANSI_RED_TEXT: &str = "\x1b[31m";
// pub const ANSI_GREEN_TEXT: &str = "\x1b[32m";
// pub const ANSI_YELLOW_TEXT: &str = "\x1b[33m";
// pub const ANSI_BLUE_TEXT: &str = "\x1b[34m";
// pub const ANSI_MAGENTA_TEXT: &str = "\x1b[35m";
// pub const ANSI_CYAN_TEXT: &str = "\x1b[36m";
// pub const ANSI_WHITE_TEXT: &str = "\x1b[37m";

// Backgrounds
// pub const ANSI_BLACK_BG: &str = "\x1b[40m";
// pub const ANSI_RED_BG: &str = "\x1b[41m";
// pub const ANSI_GREEN_BG: &str = "\x1b[42m";
// pub const ANSI_YELLOW_BG: &str = "\x1b[43m";
// pub const ANSI_BLUE_BG: &str = "\x1b[44m";
// pub const ANSI_MAGENTA_BG: &str = "\x1b[45m";
// pub const ANSI_CYAN_BG: &str = "\x1b[46m";
pub const ANSI_WHITE_BG: &str = "\x1b[47m";
