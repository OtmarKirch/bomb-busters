use std::collections::{HashSet, HashMap};
use std::sync::atomic::{AtomicU32, Ordering};
use rand::prelude::*;
use rust_helpers::{split_rand_hashset_eq};

use crate::elements::{init_all_cables, init_cables_in_game, get_value, get_color};

static PLAYER_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

pub struct Player {
    pub id: u32, // This is the player's ID
    pub hand: Vec<u32>, // Assuming cards are represented by u32 IDs
}

fn init_player(number_players: u32){}

fn init_cable_distribution(in_game_cables: &HashSet<u32>, number_players: u32) -> Vec<Vec<u32>> {
    let mut hands: Vec<Vec<u32>> = split_rand_hashset_eq(in_game_cables.clone(), number_players as usize);

    println!("Hands initialized: {:?}", hands);
    return hands
}

fn sort_cable_distribution(hands: &mut Vec<Vec<u32>>, all_cables: &HashMap<u32, u32>) {
    for hand in hands.iter_mut() {
        hand.sort_by_key(|id| all_cables.get(id).cloned().unwrap_or(0));
        println!("Sorted hand: {:?}", hand);
    }
}

pub struct Hand {
    cables: Vec<u32>, 
    status: Vec<CableStatus> 
}

#[derive(Clone)]
enum CableStatus {
    Hidden,
    Clue,
    Revealed,
}

fn init_hands(sorted_cable_distribution: Vec<Vec<u32>>) -> Vec<Hand> {
    sorted_cable_distribution.into_iter().map(|cables| {
        Hand {
            cables: cables.clone(),
            status: vec![CableStatus::Hidden; cables.len()],
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cable_distribution() {
        let all_cables = init_all_cables(10, 3, 2);
        let in_game_cables = init_cables_in_game(&all_cables, 2, 1);
        let hands: Vec<Vec<u32>> = init_cable_distribution(&in_game_cables, 3);
        assert!(!hands.is_empty(), "Hands should not be empty");
        let distribution = hands.iter().map(|hand| hand.len()).collect::<HashSet<_>>();
        assert!(distribution.iter().max().unwrap() - distribution.iter().min().unwrap() <= 1, "Hands should be evenly distributed");
    }

    #[test]
    fn test_sort_cable_distribution() {
        let all_cables = init_all_cables(10, 3, 2);
        let in_game_cables = init_cables_in_game(&all_cables, 2, 1);
        let mut hands: Vec<Vec<u32>> = init_cable_distribution(&in_game_cables, 3);
        assert!(!hands.is_empty(), "Hands should not be empty");

        sort_cable_distribution(&mut hands, &all_cables);
        for hand in &hands {
            let values: Vec<u32> = hand.iter().map(|id| get_value(*id, &all_cables)).collect();
            assert!(values.is_sorted(), "Hand should be sorted by values");
        }
    }

    #[test]
    fn test_init_hands() {
        let all_cables = init_all_cables(10, 3, 2);
        let in_game_cables = init_cables_in_game(&all_cables, 2, 1);
        let hands: Vec<Vec<u32>> = init_cable_distribution(&in_game_cables, 3);
        assert!(!hands.is_empty(), "Hands should not be empty");

        let mut sorted_hands = hands.clone();
        sort_cable_distribution(&mut sorted_hands, &all_cables);
        let initialized_hands = init_hands(sorted_hands);
        
        assert_eq!(initialized_hands.len(), hands.len(), "Number of hands should match");
        for hand in initialized_hands {
            assert_eq!(hand.status.len(), hand.cables.len(), "Status length should match cables length");
            assert!(hand.status.iter().all(|s| matches!(s, CableStatus::Hidden)), "All cables should start as Hidden");
        }
    }
}