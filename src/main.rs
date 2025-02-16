// Don't warn about dead code
// #![warn(dead_code)]
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
    let mut keys = keys::Keys::new();

    println!("{}", ansi_draw::ANSI_CLEAR_SCREEN); // clear screen
    println!("{}", ansi_draw::ANSI_HOME); // Go home

    let mut player1_score_card = score_card::ScoreCard::new("Joe");

    let mut dice = [
        dice::Die::new((27, 2), 0),
        dice::Die::new((27, 13), 0),
        dice::Die::new((27, 24), 0),
        dice::Die::new((27, 35), 0),
        dice::Die::new((27, 46), 0),
    ];

    // Main game loop
    loop {
        for die in &mut dice {
            die.roll();
            die.draw();
        }

        player1_score_card.score_top(1, &dice);
        player1_score_card.score_top(2, &dice);
        player1_score_card.score_top(3, &dice);
        player1_score_card.score_top(4, &dice);
        player1_score_card.score_top(5, &dice);
        player1_score_card.score_top(6, &dice);

        player1_score_card.score_3_of_kind(&dice);
        player1_score_card.score_4_of_kind(&dice);
        player1_score_card.score_full_house(&dice);
        player1_score_card.score_sm_straight(&dice);
        player1_score_card.score_lg_straight(&dice);
        player1_score_card.score_yahtzee(&dice);

        player1_score_card.draw();

        pause((33, 33));
    }
}

fn pause(point: (usize, usize)) {
    print!("{}", ansi_draw::ANSI_RESET_TEXT); // Reset any weird things we did
                                              // println!("\nPausing. Press Enter to Continue");
    ansi_draw::print_at(point, "Pausing. Press Enter to Continue\n");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}
