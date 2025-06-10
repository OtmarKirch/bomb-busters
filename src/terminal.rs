//! This module contains functions for the terminal interface of the game.
//! 

use std::collections::HashMap;

use std::io::{self, Write};

use crate::elements::*;
use crate::player::*;
use crate::actions::*;
use crate::elements::*;
use crate::game::*;

pub fn show_cable_info(all_cables: &HashMap<u32, u32>, game_meta: &GameMeta) {
    let (red_values, yellow_values) = get_yel_red_info(&all_cables);
    println!("Cable Information:");
    println!("Red cable values: {:?}, in game: {:?}", red_values, game_meta.red_keep);
    println!("Yellow cables: {:?}, in game: {:?}", yellow_values, game_meta.yellow_keep);
}

/// Shows all hands from the perspective of the current player.
pub fn show_hands(player_number: u32, hands: &Vec<Hand>, all_cables: &HashMap<u32, u32>) {
    let hand_player: Hand = hands[player_number as usize].clone();
    
    // Display the current player's hand
    println!("Player {}'s hand:", player_number);
    for n in 0..get_cables(&hand_player).len() {
        println!("Value: {}, Color: {}, Status: {:?}", 
            get_value(get_cables(&hand_player)[n], all_cables) as f32 / 10.0,
            get_color(get_cables(&hand_player)[n], all_cables),
            get_cables(&hand_player)[n]);
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

        assert_eq!(initialized_hands.len(), cable_distributions.len(), "Number of cable_distributions should match");
        let current_player = 0;

        show_hands(current_player, &initialized_hands, &all_cables);
    }
}