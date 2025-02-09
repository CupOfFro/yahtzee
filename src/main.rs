// Don't warn about dead code
#![warn(dead_code)]
#![allow(unused)]

use std::io;
use std::io::Write; // Bring flush into scope

// For sleep
use std::thread::sleep;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::*;

mod ansi_draw;
mod dice;
mod keys;

use crate::ansi_draw::*;

// usefule references for ANSI escape codes
// https://stackoverflow.com/questions/33139248/i-cannot-print-color-escape-codes-to-the-terminal
// https://stackoverflow.com/questions/69597466/move-cursor-escape-sequence
// https://duffney.io/usingansiescapesequencespowershell/
// https://www.lihaoyi.com/post/BuildyourownCommandLinewithANSIescapecodes.html
// https://en.wikipedia.org/wiki/ANSI_escape_code

// Info about format args for rust
// https://doc.rust-lang.org/std/fmt/index.html

fn main() {
    println!("{}", ANSI_CLEAR_SCREEN); // clear screen
    println!("{}", ANSI_HOME); // Go home

    // loop {
    //     unsafe {
    //         // let keys: &mut [u8; 256] = &mut [0; 256];
    //         // GetKeyboardState(keys);
    //         // \x1b[2K erases the whole line
    //         print!("\x1b[1;1H\x1b[2K");
    //         print!("{}", GetKeyState(VK_UP.0 as i32));
    //         print!("\x1b[2;1H\x1b[2K");
    //         print!("{}", GetKeyState(VK_DOWN.0 as i32));
    //         print!("\x1b[3;1H\x1b[2K");
    //         print!("{}", GetKeyState(VK_LEFT.0 as i32));
    //         print!("\x1b[4;1H\x1b[2K");
    //         print!("{}", GetKeyState(VK_RIGHT.0 as i32));
    //         println!("");
    //     }
    // }

    // pause();

    // println!("{}{}Hello, world!", ANSI_BLUE_BG, ANSI_GREEN_TEXT); // Red text
    // println!("\x1b[5;5HTesting"); // Go to position 5, 5 (1 based I believe)
    // println!("\x1b[31mHello, world!\x1b[39m\n"); // Red text
    // println!("\x1b[4AEka Eka Boo mean that{}", ANSI_RESET_TEXT);

    // fake_percentage();
    // println!("");
    // fake_loading_bar();

    dice::draw_die_face(dice::ONE_FACE, (10, 10));
    dice::draw_die_face(dice::TWO_FACE, (10, 21));
    dice::draw_die_face(dice::THREE_FACE, (10, 32));
    dice::draw_die_face(dice::FOUR_FACE, (10, 43));
    dice::draw_die_face(dice::FIVE_FACE, (10, 54));
    dice::draw_die_face(dice::SIX_FACE, (10, 65));

    draw_horizontal_line(&mut Point { r: 1, c: 1 }, "%", 100);
    draw_vertical_line(&mut Point { r: 1, c: 1 }, "%", 50);
    draw_horizontal_line(&mut Point { r: 2, c: 2 }, "#", 100);
    draw_to_screen();

    pause();
}

fn pause() {
    print!("{}", ANSI_RESET_TEXT); // Reset any weird things we did
    println!("\nPausing. Press Enter to Continue");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}

// This will draw any print statements we have done with no new lines
fn draw_to_screen() {
    io::stdout().flush().expect("Welp this is bad");
}

fn draw_horizontal_line(p: &mut Point, char: &str, length: usize) {
    // I could use format to do this but think it looks better to use ANSI escapes like most of program
    for i in 0..length {
        let position = format!("\x1b[{};{}H", &p.r, &p.c);
        p.c += 1;
        print!("{}{}", position, char);
    }
}

fn draw_vertical_line(p: &mut Point, char: &str, length: usize) {
    // I could use format to do this but think it looks better to use ANSI escapes like most of program
    for i in 0..length {
        let position = format!("\x1b[{};{}H", &p.r, &p.c);
        p.r += 1;
        print!("{}{}", position, char);
    }
}

// Moving Cursor
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
        sleep(Duration::from_millis(100));
    }
}

fn fake_loading_bar() {
    for i in 1..=20 {
        // So this is a bit complicated at first but relatively simple
        // We start with "\x1b[100D[", this moves the cursor back either
        // 100 spaces, or to the left side of the screen, and will then
        // redraw from there.
        // So {} in print! will print an arg after the closing ".
        // : inside {} '{:}' gives us various format options
        // :# tells it to fill whitespace with a '#' char.
        // < says to left justify any fill space.
        // 1$ is saying that argument 1 (arguments are 0 based) is how much space to fill with.
        // so this matters because {} automatically assumes we are fillign with arg 0,
        // which in this case is "" an empty string.
        // Next we print argument 2 a "]" right justified
        print!("\x1b[100D[{:#<1$}{2:>3$}", "", i, "]", 21 - i);
        io::stdout().flush().expect("Welp this is bad");
        sleep(Duration::from_millis(100));
    }
}
