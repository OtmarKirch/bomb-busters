use std::collections::{HashSet, HashMap};
use std::sync::atomic::{AtomicU32, Ordering};
use rand::prelude::*;
use rust_helpers::{split_rand_hashset_eq};

use crate::elements::{init_all_cables, init_cables_in_game, get_value, get_color};

pub fn init_cable_distribution(in_game_cables: &HashSet<u32>, number_players: u32) -> Vec<Vec<u32>> {
    let mut distributions: Vec<Vec<u32>> = split_rand_hashset_eq(in_game_cables.clone(), number_players as usize);

    println!("Hands initialized: {:?}", distributions);
    return distributions
}

pub fn sort_cable_distribution(distributions: &mut Vec<Vec<u32>>, all_cables: &HashMap<u32, u32>) {
    for distribution in distributions.iter_mut() {
        distribution.sort_by_key(|id| all_cables.get(id).cloned().unwrap_or(0));
        println!("Sorted diestribution: {:?}", distribution);
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    cables: Vec<u32>, 
    status: Vec<CableStatus> 
}

impl Hand {
    pub fn new(cables: Vec<u32>) -> Self {
        let status = vec![CableStatus::Hidden; cables.len()];
        Hand { cables, status }
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum CableStatus {
    Hidden,
    Clue,
    Revealed,
}

pub fn get_cables(hand: &Hand) -> Vec<u32> {
    hand.cables.clone()
}

pub fn get_status(hand: &Hand) -> Vec<CableStatus> {
    hand.status.clone()
}

pub fn init_hands(sorted_cable_distribution: Vec<Vec<u32>>) -> Vec<Hand> {
    sorted_cable_distribution.into_iter().map(|cables| {
        Hand {
            cables: cables.clone(),
            status: vec![CableStatus::Hidden; cables.len()],
        }
    }).collect()
}

pub fn get_teammate_hands(player_number: u32, hands: &Vec<Hand>) -> Vec<Hand> {
    let teammate_hands: Vec<Hand> = hands.iter()
        .enumerate()
        .filter(|(index, _)| *index as u32 != player_number)
        .map(|(_, hand)| hand.clone())
        .collect();

    for hand in &teammate_hands {       
    println!("Teammate's hand {:?}", hand);
    }
    return teammate_hands
}

pub fn change_cable_status(cable_id: u32, hands: &mut Vec<Hand>, new_status: CableStatus) {
    for hand in hands.iter_mut() {
        if let Some(pos) = hand.cables.iter().position(|&id| id == cable_id) {
            hand.status[pos] = new_status;
            println!("Changed status of cable {} to {:?}", cable_id, new_status);
        }
    }
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
        let mut cable_distributions: Vec<Vec<u32>> = init_cable_distribution(&in_game_cables, 3);
        assert!(!cable_distributions.is_empty(), "Hands should not be empty");

        sort_cable_distribution(&mut cable_distributions, &all_cables);
        for cable_distribution in &cable_distributions {
            let values: Vec<u32> = cable_distribution.iter().map(|id| get_value(*id, &all_cables)).collect();
            assert!(values.is_sorted(), "Hand should be sorted by values");
        }
    }

    #[test]
    fn test_init_hands() {
        let all_cables = init_all_cables(10, 3, 2);
        let in_game_cables = init_cables_in_game(&all_cables, 2, 1);
        let cable_distributions: Vec<Vec<u32>> = init_cable_distribution(&in_game_cables, 3);
        assert!(!cable_distributions.is_empty(), "Hands should not be empty");

        let mut sorted_hands = cable_distributions.clone();
        sort_cable_distribution(&mut sorted_hands, &all_cables);
        let initialized_hands = init_hands(sorted_hands);
        
        assert_eq!(initialized_hands.len(), cable_distributions.len(), "Number of cable_distributions should match");
        for hand in initialized_hands {
            assert_eq!(hand.status.len(), hand.cables.len(), "Status length should match cables length");
            assert!(hand.status.iter().all(|s| matches!(s, CableStatus::Hidden)), "All cables should start as Hidden");
        }
    }

    #[test]
    fn test_get_teammate_hands() {
        let all_cables = init_all_cables(10, 3, 2);
        let in_game_cables = init_cables_in_game(&all_cables, 2, 1);
        let cable_distributions: Vec<Vec<u32>> = init_cable_distribution(&in_game_cables, 3);
        assert!(!cable_distributions.is_empty(), "Hands should not be empty");

        let mut sorted_hands = cable_distributions.clone();
        sort_cable_distribution(&mut sorted_hands, &all_cables);
        let initialized_hands = init_hands(sorted_hands);
        
        assert_eq!(initialized_hands.len(), cable_distributions.len(), "Number of cable_distributions should match");
        let current_player = 1;

        get_teammate_hands(current_player, &initialized_hands);
    }

    #[test]
    fn test_change_cable_status() {
        let mut hands = vec![
            Hand {
                cables: vec![1, 2, 3],
                status: vec![CableStatus::Clue, CableStatus::Hidden, CableStatus::Revealed],
            },
            Hand {
                cables: vec![4, 5, 6],
                status: vec![CableStatus::Hidden; 3],
            },
        ];
        change_cable_status(2, &mut hands, CableStatus::Revealed);
        assert_eq!(hands[0].status[1], CableStatus::Revealed, "Cable status should be changed to Revealed");
        change_cable_status(4, &mut hands, CableStatus::Clue);
        assert_eq!(hands[1].status[0], CableStatus::Clue, "Cable status should be changed to Clue");
        //let mut all_cables: HashMap<u32, u32> = HashMap::new();
        //all_cables.extend([(1, 10), (2, 25), (3, 30), (4, 40), (5, 51), (6, 60)]);
    }
}