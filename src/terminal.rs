//! This module contains functions for the terminal interface of the game.
//!

use std::collections::HashMap;

use std::io::{self, Write};

use crate::actions::*;
use crate::elements::*;
use crate::game::*;
use crate::player::*;

pub fn create_new_game() {
    println!("Welcome to Bomb Buster!");
    io::stdout().flush().unwrap();

    let num_players = def_number_players();
    let max_blue_cables = def_max_blue_cables();
    let max_red_cables = def_max_red_cables();
    let keep_red_cables = def_keep_red_cables(max_red_cables);
    let max_yellow_cables = def_max_yellow_cables();
    let keep_yellow_cables = def_keep_yellow_cables(max_yellow_cables);

    println!(
        "Starting a new game with {} players, max blue cables: {}, max red cables: {}, keep red cables: {}, max yellow cables: {}, keep yellow cables: {}",
        num_players, max_blue_cables, max_red_cables, keep_red_cables, max_yellow_cables, keep_yellow_cables
    );
}

pub fn def_number_players() -> u32 {
    loop {
        print!("Enter the number of players (2-4): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(num) if num >= 2 && num <= 4 => return num,
                _ => println!("Please enter a valid number between 2 and 4."),
            },
            Err(_) => println!("Failed to read input. Please try again."),
        }
    }
}

pub fn def_max_blue_cables() -> u32 {
    loop {
        print!("Enter the maximum value for blue cables (8-12): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(num) if num >= 8 && num <= 12 => return num,
                _ => println!("Please enter a valid number between 8 and 12."),
            },
            Err(_) => println!("Failed to read input. Please try again."),
        }
    }
}

pub fn def_max_red_cables() -> u32 {
    loop {
        print!("Enter the number of red cables to show (0-3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(num) if num >= 0 && num <= 3 => return num,
                _ => println!("Please enter a valid number between 0 and 3."),
            },
            Err(_) => println!("Failed to read input. Please try again."),
        }
    }
}

pub fn def_keep_red_cables(max_red_cables: u32) -> u32 {
    match max_red_cables {
        0 => 0,
        1 => 1,
        _ => loop {
            print!(
                "Enter the number of red cables to keep (0-{}): ",
                max_red_cables
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim().parse::<u32>() {
                    Ok(num) if num <= max_red_cables => return num,
                    _ => println!(
                        "Please enter a valid number between 0 and {}.",
                        max_red_cables
                    ),
                },
                Err(_) => println!("Failed to read input. Please try again."),
            }
        },
    }
}

pub fn def_max_yellow_cables() -> u32 {
    loop {
        print!("Enter the number of yellow cables to show (0, 2 or 3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<u32>() {
                Ok(num) if num == 0 || num == 2 || num == 3 => return num,
                _ => println!("Please enter 0, 2 or 3."),
            },
            Err(_) => println!("Failed to read input. Please try again."),
        }
    }
}

pub fn def_keep_yellow_cables(max_yellow_cables: u32) -> u32 {
    if max_yellow_cables == 0 || max_yellow_cables == 2 {
        return max_yellow_cables;
    }
    match max_yellow_cables {
        0 => 0,
        _ => loop {
            print!("Enter the number of yellow cables to keep (2 or 3): " );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.trim().parse::<u32>() {
                    Ok(num) if num == 2 || num == 3 => return num,
                    _ => println!("Please enter 2 or 3."),
                },
                Err(_) => println!("Failed to read input. Please try again."),
            }
        },
    }
}

pub fn show_cable_info(all_cables: &HashMap<u32, u32>, game_meta: &GameMeta) {
    let (red_values, yellow_values) = get_yel_red_info(&all_cables);
    println!("Cable Information:");
    println!(
        "Red cable values: {:?}, in game: {:?}",
        red_values, game_meta.red_keep
    );
    println!(
        "Yellow cables: {:?}, in game: {:?}",
        yellow_values, game_meta.yellow_keep
    );
}

/// Shows all hands from the perspective of the current player.
pub fn show_hands(player_number: u32, hands: &Vec<Hand>, all_cables: &HashMap<u32, u32>) {
    let hand_player: Hand = hands[player_number as usize].clone();

    // Display the current player's hand
    println!("Player {}'s hand:", player_number);
    for n in 0..get_cables(&hand_player).len() {
        println!(
            "Value: {}, Color: {}, Status: {:?}",
            get_value(get_cables(&hand_player)[n], all_cables) as f32 / 10.0,
            get_color(get_cables(&hand_player)[n], all_cables),
            get_cables(&hand_player)[n]
        );
    }

    // Display the hands of other players
    for (i, hand) in hands.iter().enumerate() {
        if i as u32 != player_number {
            println!("Player {}'s hand:", i);
            for n in 0..get_cables(&hand).len() {
                let status = match get_status(hand)[n] {
                    CableStatus::Hidden => "Hidden",
                    CableStatus::Clue => "Clue",
                    CableStatus::Revealed => "Revealed",
                };
                let color = match status {
                    "Hidden" => "Hidden".to_string(),
                    _ => get_color(get_cables(&hand)[n], all_cables),
                };
                let value = match status {
                    "Hidden" => "Hidden".to_string(),
                    _ => (get_value(get_cables(&hand)[n], all_cables) as f32 / 10.0).to_string(),
                };
                println!("Value: {}, Color: {}, Status: {:?}", value, color, status);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_hands() {
        let all_cables = init_all_cables(10, 3, 2);
        let in_game_cables = init_cables_in_game(&all_cables, 2, 1);
        let cable_distributions: Vec<Vec<u32>> = init_cable_distribution(&in_game_cables, 3);
        assert!(!cable_distributions.is_empty(), "Hands should not be empty");

        let mut sorted_hands = cable_distributions.clone();
        sort_cable_distribution(&mut sorted_hands, &all_cables);
        let initialized_hands = init_hands(sorted_hands);

        assert_eq!(
            initialized_hands.len(),
            cable_distributions.len(),
            "Number of cable_distributions should match"
        );
        let current_player = 0;

        show_hands(current_player, &initialized_hands, &all_cables);
    }
}
