mod elements;
mod player;
mod actions;

fn main() {
    println!("This is Bomb Buster!");
    let (blue_max, red_show, red_keep, yellow_show, yellow_keep, players) = (12, 2, 1, 3, 2, 3);
    let all_cables = elements::init_all_cables(blue_max, red_show, yellow_show);
    let in_game_cables = elements::init_cables_in_game(&all_cables, red_keep, yellow_keep);
    let mut cable_distribution = player::init_cable_distribution(&in_game_cables, players);
    player::sort_cable_distribution(&mut cable_distribution, &all_cables);
    let hands = player::init_hands(cable_distribution);
    let current_player = 0; // Assuming player 0 is the current player
    player::show_hands(current_player, &hands, &all_cables);
}
