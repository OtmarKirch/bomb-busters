//! This module contains functions for the terminal interface of the game.
//! 

use std::collections::HashMap;

use std::io::{self, Write};

use crate::{elements::*, game};
use crate::player::*;
use crate::actions::*;

pub fn show_cable_info(all_cables: &HashMap<u32, u32>, game_meta: &game::GameMeta) {
    let (red_values, yellow_values) = get_yel_red_info(&all_cables);
    println!("Cable Information:");
    println!("Red cable values: {:?}, in game: {:?}", red_values, game_meta.red_keep);
    println!("Yellow cables: {:?}, in game: {:?}", yellow_values, game_meta.yellow_keep);
}