// Don't warn about dead code
#![warn(dead_code)]
#![allow(unused)]

use std::io;
use std::io::Write; // Bring flush into scope

// For sleep
use std::thread::sleep;
use std::time::Duration;

mod ansi_draw;
mod dice;
mod keys;

fn main() {
    println!("{}", ansi_draw::ANSI_CLEAR_SCREEN); // clear screen
    println!("{}", ansi_draw::ANSI_HOME); // Go home

    ansi_draw::print_at((5, 5), "Hello There!");

    // pause();

    // println!("{}{}Hello, world!", ANSI_BLUE_BG, ANSI_GREEN_TEXT); // Red text
    // println!("\x1b[5;5HTesting"); // Go to position 5, 5 (1 based I believe)
    // println!("\x1b[31mHello, world!\x1b[39m\n"); // Red text
    // println!("\x1b[4AEka Eka Boo mean that{}", ANSI_RESET_TEXT);

    // fake_percentage();
    // println!("");
    // fake_loading_bar();

    let die_1 = dice::Die::new((10, 10), 0);
    let die_2 = dice::Die::new((10, 21), 0);
    let die_3 = dice::Die::new((10, 32), 0);
    let die_4 = dice::Die::new((10, 43), 0);
    let die_5 = dice::Die::new((10, 54), 0);

    die_1.draw();
    die_2.draw();
    die_3.draw();
    die_4.draw();
    die_5.draw();

    ansi_draw::draw_horizontal_line((1, 1), "%", 100);
    ansi_draw::draw_vertical_line((1, 1), "%", 50);
    ansi_draw::draw_horizontal_line((1, 1), "#", 100);
    ansi_draw::draw_to_screen();

    pause();
}

fn pause() {
    print!("{}", ansi_draw::ANSI_RESET_TEXT); // Reset any weird things we did
    println!("\nPausing. Press Enter to Continue");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
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
