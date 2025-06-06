use crate::elements::*;
use crate::player::*;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

fn hint_random(hands: &mut Vec<Hand>) {
    let mut rng = rand::rng();
    let mut cable_ids: HashSet<u32> = HashSet::new();
    for hand in hands.iter() {
        let (_, &cable_id) = get_cables(hand).iter().enumerate().choose(&mut rng).unwrap();
        cable_ids.insert(cable_id);
    }
    for id in cable_ids {
        change_cable_status(id, hands, CableStatus::Clue);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hint_random() {
        let mut hands = vec![
            Hand::new(vec![1, 2, 3]),
            Hand::new(vec![4, 5, 6, 7]),
        ];
        hint_random(&mut hands);

        for i in 0..hands.len() {
            let cnt_clue = get_status(&hands[i]).iter().filter(|&&status| status == CableStatus::Clue).count();
            
            assert!(cnt_clue == 1, "Expected exactly one cable to be changed in player {}'s hand, caught {}", i, cnt_clue);
        }
    }
}
