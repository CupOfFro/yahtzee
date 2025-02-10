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
mod score_card;

fn main() {
    println!("{}", ansi_draw::ANSI_CLEAR_SCREEN); // clear screen
    println!("{}", ansi_draw::ANSI_HOME); // Go home

    let player1_score_card = score_card::ScoreCard::new("Joe");

    let mut dice = vec![
        dice::Die::new((25, 2), 0),
        dice::Die::new((25, 13), 0),
        dice::Die::new((25, 24), 0),
        dice::Die::new((25, 35), 0),
        dice::Die::new((25, 46), 0),
    ];

    // Main game loop
    loop {
        player1_score_card.draw();
        for die in &mut dice {
            die.roll();
            die.draw();
        }
        ansi_draw::draw_to_screen();

        pause();
    }
}

fn pause() {
    print!("{}", ansi_draw::ANSI_RESET_TEXT); // Reset any weird things we did
                                              // println!("\nPausing. Press Enter to Continue");
    ansi_draw::print_at((30, 20), "Pausing. Press Enter to Continue\n");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}

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
