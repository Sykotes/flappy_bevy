use bevy::prelude::*;

pub struct GameData {
    pub gamestate: Game,
    pub score: u32,
}

#[derive(PartialEq)]
pub enum Game{
    Opened,
    Running,
    Over,
}

#[derive(Resource)]
pub struct GameState(pub GameData);

fn handle_gamestates(
    gamestate: ResMut<GameState>
) {
    if gamestate.0.gamestate == Game::Over {
        println!("Score: {}", gamestate.0.score);
    }
}
