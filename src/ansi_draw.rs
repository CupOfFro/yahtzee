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

// Moving Cursor
// Up: \x1b[{n}A
// Down: \x1b[{n}B
// Right: \x1b[{n}C
// Left: \x1b[{n}D

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

pub fn print_at(point: (usize, usize), words: &str) {
    let (term_row, term_col) = point;
    let position = format!("\x1b[{};{}H", term_row, term_col);
    print!("{}{}", position, words);
}

pub const ANSI_CLEAR_SCREEN: &str = "\x1b[2J";
pub const ANSI_HOME: &str = "\x1b[H";
pub const ANSI_RESET_TEXT: &str = "\x1b[0m";
pub const ANSI_ERASE_LINE: &str = "\x1b[2K";

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
pub const ANSI_BLUE_BG: &str = "\x1b[44m";
// pub const ANSI_MAGENTA_BG: &str = "\x1b[45m";
// pub const ANSI_CYAN_BG: &str = "\x1b[46m";
pub const ANSI_WHITE_BG: &str = "\x1b[47m";

// fn fake_percentage() {
//     for i in 1..=100 {
//         print!("\x1b[5D{i}%");
//         // We need to flush to get the terminal to actually draw without the newline
//         // This is good to know cause now I think I can draw a whole screen and update
//         // it with flush
//         io::stdout().flush().expect("Welp this is bad");
//         sleep(Duration::from_millis(100));
//     }
// }

// fn fake_loading_bar() {
//     for i in 1..=20 {
//         // So this is a bit complicated at first but relatively simple
//         // We start with "\x1b[100D[", this moves the cursor back either
//         // 100 spaces, or to the left side of the screen, and will then
//         // redraw from there.
//         // So {} in print! will print an arg after the closing ".
//         // : inside {} '{:}' gives us various format options
//         // :# tells it to fill whitespace with a '#' char.
//         // < says to left justify any fill space.
//         // 1$ is saying that argument 1 (arguments are 0 based) is how much space to fill with.
//         // so this matters because {} automatically assumes we are fillign with arg 0,
//         // which in this case is "" an empty string.
//         // Next we print argument 2 a "]" right justified
//         print!("\x1b[100D[{:#<1$}{2:>3$}", "", i, "]", 21 - i);
//         io::stdout().flush().expect("Welp this is bad");
//         sleep(Duration::from_millis(100));
//     }
// }