//! This module contains all the elements the game uses, such as cables and equipment.
//! Note that cable values are represented as integers. A cable in the game with the number 4.5 is represented as 45 in the code. This is to avoid floating point precision issues.
//! The collections should be initialized once immutably at the start of the game with the init functions. Other functions refer to these collections by unique IDs.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use rand::prelude::*;
use rust_helpers::{split_rand_vec, split_rand_vec_eq};

static CABLE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

/// Initializes all cables with a given configuration.
///
/// # Arguments
/// * `blue_max_val` - The maximum value for blue cables. There are 4 cables for each value.
/// * `red_show` - The number of red cables. These are initially shown to the players.
/// * `yellow_show` - The number of yellow cables. These are initially shown to the players.
///
/// # Returns
/// A `HashMap` where the keys are cable IDs and the values are the cable values.
fn init_all_cables(blue_max_val: u32, red_show: u32, yellow_show: u32) -> HashMap<u32, u32> {
    let mut cables = HashMap::new();
    for i in 1..=blue_max_val {
        for _ in 1..=4 {
            cables.insert(CABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed), i as u32 * 10);
        }
    }
    for j in 1..=red_show {
        cables.insert(CABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed), j as u32 * 10 + 5);
    }
    for k in 1..=yellow_show {
        cables.insert(CABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed), k as u32 * 10 + 1);
    }

    for cable in cables.iter() {
        println!("Cable ID: {}, Value: {}", cable.0, cable.1);
    }

    return cables
}

fn init_cables_in_game(
    cables: &HashMap<u32, u32>,
    red_keep: u32,
    yellow_keep: u32,
) -> HashSet<u32> {
    let mut in_game_cables: HashSet<u32> = HashSet::new();
    let blue_cable_ids: Vec<u32> = cables
        .iter()
        .filter(|(_, v)| *v % 10 == 0)
        .map(|(k, _)| *k)
        .collect();
    println!("Blue cable IDs: {:?}, length: {}", blue_cable_ids, blue_cable_ids.len());
    in_game_cables.extend(blue_cable_ids);

    let mut rng = rand::rng();
    
    let red_cable_ids: Vec<u32> = cables
        .iter()
        .filter(|(_, v)| *v % 10 == 5)
        .choose_multiple(&mut rng, red_keep as usize).into_iter()
        .map(|(k, _)| *k)
        .collect();
    println!("Red cable IDs: {:?}, length: {}", red_cable_ids, red_cable_ids.len());
    in_game_cables.extend(red_cable_ids);

    let yellow_cable_ids: Vec<u32> = cables
        .iter()
        .filter(|(_, v)| *v % 10 == 1)
        .choose_multiple(&mut rng, yellow_keep as usize).into_iter()
        .map(|(k, _)| *k)
        .collect();
    println!("Yellow cable IDs: {:?}, length: {}", yellow_cable_ids, yellow_cable_ids.len());
    in_game_cables.extend(yellow_cable_ids);
    
    return in_game_cables

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_all_cables() {
        let blue_max_val = 12;
        let red_show = 2;
        let yellow_show = 3;

        let cables = init_all_cables(blue_max_val, red_show, yellow_show);

        let num_blue_cables = cables.iter().filter(|(_, v)| *v % 10 == 0).count();
        let num_red_cables = cables.iter().filter(|(_, v)| *v % 10 == 5).count();
        let num_yellow_cables = cables.iter().filter(|(_, v)| *v % 10 == 1).count();

        assert_eq!(cables.len(), (4 * blue_max_val + red_show + yellow_show) as usize, "Expected {} cables, found {}", 4 * blue_max_val + red_show + yellow_show, cables.len());
        assert_eq!(num_blue_cables, 4 * blue_max_val as usize, "Expected {} blue cables, found {}", 4 * blue_max_val, num_blue_cables);
        assert_eq!(num_red_cables, red_show as usize, "Expected {} red cables, found {}", red_show, num_red_cables);
        assert_eq!(num_yellow_cables, yellow_show as usize, "Expected {} yellow cables, found {}", yellow_show, num_yellow_cables);
    }

    #[test]
    fn test_init_cables_in_game() {
        let cables = init_all_cables(12, 2, 3);
        let (red_keep, yellow_keep) = (1, 2);
        let in_game_cables = init_cables_in_game(&cables, red_keep, yellow_keep);
        let expected_cables = 4 * 12 + red_keep + yellow_keep;
        assert_eq!(in_game_cables.len(), expected_cables as usize, "Expected {} cables in game, found {}", expected_cables, in_game_cables.len());

        let num_blue_cables = in_game_cables.iter().filter(|&&id| cables[&id] % 10 == 0).count();
        let num_red_cables = in_game_cables.iter().filter(|&&id| cables[&id] % 10 == 5).count();
        let num_yellow_cables = in_game_cables.iter().filter(|&&id| cables[&id] % 10 == 1).count();
        assert_eq!(num_blue_cables, 4 * 12, "Expected {} blue cables in game, found {}", 4 * 12, num_blue_cables);
        assert_eq!(num_red_cables, red_keep as usize, "Expected {} red cables in game, found {}", red_keep, num_red_cables);
        assert_eq!(num_yellow_cables, yellow_keep as usize, "Expected {} yellow cables in game, found {}", yellow_keep, num_yellow_cables);
    }
}

