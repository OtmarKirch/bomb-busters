//! This module contains all the elements the game uses, such as cables and equipment.
//! Note that cable values are represented as integers. A cable in the game with the number 4.5 is represented as 45 in the code. This is to avoid floating point precision issues.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

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
        for j in 1..=4 {
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

fn select_cable(
    cables: &HashMap<u32, f32>,
    red_keep: u32,
    yellow_keep: u32,
) -> (HashMap<u32, f32>, HashMap<u32, f32>) {
    todo!()
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
}

