

mod game;
mod elements;
mod player;
mod actions;
mod terminal;

use crate::terminal::*;
use crate::actions::*;

fn main() {
    println!("This is Bomb Buster!");
    let (blue_max, red_show, red_keep, yellow_show, yellow_keep, players) = (12, 2, 1, 3, 2, 3);
    let game = game::GameMeta::new(blue_max, red_show, red_keep, yellow_show, yellow_keep, players);
    let death_counter = elements::init_death_counter(players);
    println!("Death counter: {}", death_counter);
    let all_cables = elements::init_all_cables(blue_max, red_show, yellow_show);
    let in_game_cables = elements::init_cables_in_game(&all_cables, red_keep, yellow_keep);
    let mut cable_distribution = player::init_cable_distribution(&in_game_cables, players);
    player::sort_cable_distribution(&mut cable_distribution, &all_cables);
    let mut hands = player::init_hands(cable_distribution);
    let current_player = 0; // Assuming player 0 is the current player
    hint_random(&mut hands);
    show_cable_info(&all_cables, &game);
    show_hands(current_player, &hands, &all_cables);
}
