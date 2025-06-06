pub struct GameMeta {
    pub blue_max: u32,
    pub red_show: u32,
    pub red_keep: u32,
    pub yellow_show: u32,
    pub yellow_keep: u32,
    pub players: u32,
}

impl GameMeta {
    pub fn new(blue_max: u32, red_show: u32, red_keep: u32, yellow_show: u32, yellow_keep: u32, players: u32) -> Self {
        GameMeta {
            blue_max,
            red_show,
            red_keep,
            yellow_show,
            yellow_keep,
            players,
        }
    }

    pub fn init_game_meta() -> Self {
        let blue_max = 12;
        let red_show = 2;
        let red_keep = 1;
        let yellow_show = 3;
        let yellow_keep = 2;
        let players = 3;

        GameMeta::new(blue_max, red_show, red_keep, yellow_show, yellow_keep, players)
    }
}