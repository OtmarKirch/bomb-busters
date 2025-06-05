use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

static CABLE_ID_COUNTER: AtomicU32 = AtomicU32::new(1);

fn init_all_cables(blue_max_val: u32, red_show: u32, yellow_show: u32) -> HashMap<u32, f32> {
    let mut cables = HashMap::new();
    for i in 1..=blue_max_val {
        for j in 1..=4 {
            cables.insert(CABLE_ID_COUNTER.fetch_add(1, Ordering::Relaxed), i as f32);
        }
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

        let num_blue_cables = cables.iter().filter(|(_, v)| v.fract() == 0.0).count();
        let num_red_cables = cables.iter().filter(|(_, v)| v.fract() == 0.5).count();
        let num_yellow_cables = cables.iter().filter(|(_, v)| v.fract() == 0.1).count();

        assert_eq!(num_blue_cables, 48);
        assert_eq!(num_red_cables, 2);
        assert_eq!(num_yellow_cables, 3);
    }
}

