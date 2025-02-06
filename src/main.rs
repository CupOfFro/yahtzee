// Don't warn about dead code
#![warn(dead_code)]
#![allow(unused)]

use std::io;
use std::io::Write; // Bring fush into scope

// For sleep
use std::thread::sleep;
use std::time::Duration;

// usefule references
// https://stackoverflow.com/questions/33139248/i-cannot-print-color-escape-codes-to-the-terminal
// https://stackoverflow.com/questions/69597466/move-cursor-escape-sequence
// https://duffney.io/usingansiescapesequencespowershell/
// https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html

fn main() {
    println!("{}", ANSI_CLEAR_SCREEN); // clear screen
    println!("{}", ANSI_HOME); // Go home

    // println!("{}{}Hello, world!", ANSI_BLUE_BG, ANSI_GREEN_TEXT); // Red text
    // println!("\x1b[5;5HTesting"); // Go to position 5, 5 (1 based I believe)
    // println!("\x1b[31mHello, world!\x1b[39m\n"); // Red text
    // println!("\x1b[4AEka Eka Boo mean that{}", ANSI_RESET_TEXT);

    fake_percentage();
    fake_loading_bar();

    pause();
}

// Up: \x1b[{n}A
// Down: \x1b[{n}B
// Right: \x1b[{n}C
// Left: \x1b[{n}D

fn fake_percentage() {
    for i in 1..=100 {
        print!("\x1b[5D{i}%");
        // We need to flush to get the terminal to actually draw without the newline
        // This is good to know cause now I think I can draw a whole screen and update
        // it with flush
        io::stdout().flush().expect("Welp this is bad");
        // sleep(Duration::from_millis(100));
    }
}

fn fake_loading_bar() {
    for i in 1..=20 {
        print!("\x1b[100D[{:#<1$}{:]>20}", "", i);
        io::stdout().flush().expect("Welp this is bad");
        sleep(Duration::from_millis(100));
    }
}

fn pause() {
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}

// Moving Cursor

const ANSI_CLEAR_SCREEN: &str = "\x1b[2J";
const ANSI_HOME: &str = "\x1b[H";

const ANSI_RESET_TEXT: &str = "\x1b[0m";

// Colors
const ANSI_BLACK_TEXT: &str = "\x1b[30m";
const ANSI_RED_TEXT: &str = "\x1b[31m";
const ANSI_GREEN_TEXT: &str = "\x1b[32m";
const ANSI_YELLOW_TEXT: &str = "\x1b[33m";
const ANSI_BLUE_TEXT: &str = "\x1b[34m";
const ANSI_MAGENTA_TEXT: &str = "\x1b[35m";
const ANSI_CYAN_TEXT: &str = "\x1b[36m";
const ANSI_WHITE_TEXT: &str = "\x1b[37m";

// Backgrounds

const ANSI_BLACK_BG: &str = "\x1b[40m";
const ANSI_RED_BG: &str = "\x1b[41m";
const ANSI_GREEN_BG: &str = "\x1b[42m";
const ANSI_YELLOW_BG: &str = "\x1b[43m";
const ANSI_BLUE_BG: &str = "\x1b[44m";
const ANSI_MAGENTA_BG: &str = "\x1b[45m";
const ANSI_CYAN_BG: &str = "\x1b[46m";
const ANSI_WHITE_BG: &str = "\x1b[47m";
