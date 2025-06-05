use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use rand::prelude::*;
use rust_helpers::{split_rand_vec, split_rand_vec_eq};

use crate::elements::{init_all_cables, init_cables_in_game};

static PLAYER_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

pub struct Player {
    pub id: u32, // This is the player's ID
    pub hand: Vec<u32>, // Assuming cards are represented by u32 IDs
}

fn init_player(number_players: u32){}

fn init_hands(in_game_cables: &HashSet<u32>, number_players: u32) -> HashSet<Vec<u32>> {
    return HashSet::<Vec<u32>>::new();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_hands() {
        let all_cables = init_all_cables(10, 3, 2);
        let in_game_cables = init_cables_in_game(&all_cables, 2, 1);
        let hands: HashSet<Vec<u32>> = init_hands(&in_game_cables, 3);
        assert!(!hands.is_empty(), "Hands should not be empty");
        let distribution = hands.iter().map(|hand| hand.len()).collect::<HashSet<_>>();
        assert!(distribution.iter().max().unwrap() - distribution.iter().min().unwrap() <= 1, "Hands should be evenly distributed");
    }

}