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

    let mut die_selected = 0;
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
            die.draw(false);
        }
        dice[die_selected].draw(true);

        // player1_score_card.score_top(1, &dice);
        // player1_score_card.score_top(2, &dice);
        // player1_score_card.score_top(3, &dice);
        // player1_score_card.score_top(4, &dice);
        // player1_score_card.score_top(5, &dice);
        // player1_score_card.score_top(6, &dice);

        // player1_score_card.score_3_of_kind(&dice);
        // player1_score_card.score_4_of_kind(&dice);
        // player1_score_card.score_full_house(&dice);
        // player1_score_card.score_sm_straight(&dice);
        // player1_score_card.score_lg_straight(&dice);
        // player1_score_card.score_yahtzee(&dice);

        player1_score_card.draw();

        // Wait for a key to be pressed
        while !keys.was_key_pressed() {
            keys.check_keys_toggle();
        }

        // Now handle the key being pressed
        if keys.up.1 == true {
            keys.up.1 = false;
            player1_score_card.selection -= 1;
            if player1_score_card.selection < 1 {
                player1_score_card.selection = 1;
            }
        } else if keys.down.1 == true {
            keys.down.1 = false;
            player1_score_card.selection += 1;
            if player1_score_card.selection > 14 {
                player1_score_card.selection = 14;
            }
        } else if keys.left.1 == true {
            keys.left.1 = false;
            // avoid subtract with overflow error
            if die_selected > 0 {
                die_selected -= 1;
            }
        } else if keys.right.1 == true {
            keys.right.1 = false;
            die_selected += 1;
            if die_selected > dice.len() - 1 {
                die_selected = dice.len() - 1;
            }
        } else if keys.k.1 == true {
            keys.k.1 = false;
            dice[die_selected].toggle_rollable();
        }

        // pause((33, 33));
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
