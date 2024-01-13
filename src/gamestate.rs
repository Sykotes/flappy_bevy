use bevy::prelude::*;

pub struct GameData {
    pub gamestate: Game,
    pub score: u32,
}

#[derive(PartialEq)]
pub enum Game {
    Opened,
    Running,
    Over,
}

#[derive(Resource)]
pub struct GameState(pub GameData);
