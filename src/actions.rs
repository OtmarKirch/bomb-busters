use crate::elements::*;
use crate::player::*;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn hint_random(hands: &mut Vec<Hand>) {
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

pub fn duo_cut(player: u32, position_self: u32, position_teammate: u32, hands: &mut Vec<Hand>, all_cables: &HashMap<u32, u32>) {

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

    #[test]
    fn test_duo_cut() {
        // Successfull duo cut
        let all_cables = HashMap::from([
            (1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 40), (7, 20), // blue
            (8, 15), (9, 25), // red
            (11, 11), (12, 21), (13, 31), // yellow
        ]);
        let mut hands = vec![
            Hand::new(vec![1, 2, 3, 5, 6, 8, 11]),
            Hand::new(vec![4, 7, 9, 10, 12, 13]),
        ];

        let hand0_cables = get_cables(&hands[0]);
        let hand0_status = get_status(&hands[0]);
        let hand1_cables = get_cables(&hands[1]);
        let hand1_status = get_status(&hands[1]);

        assert!(matches!(hand0_status[2], CableStatus::Hidden | CableStatus::Clue), "Expected cable at position 2 in player 0's hand to be hidden or a clue before duo cut");
        assert!(matches!(hand1_status[2], CableStatus::Hidden | CableStatus::Clue), "Expected cable at position 2 in player 1's hand to be hidden or a clue before duo cut");
        
        duo_cut(0, 2, 2, &mut hands, &all_cables);
        assert_eq!(hand0_status[2], CableStatus::Revealed, "Expected cable at position 2 in player 0's hand to be revealed after successfull duo cut");
        assert_eq!(hand1_status[2], CableStatus::Revealed, "Expected cable at position 2 in player 1's hand to be revealed after successfull duo cut");
        
        // Unsuccessful duo cut

        
    }
}
