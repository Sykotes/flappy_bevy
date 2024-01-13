use bevy::prelude::*;

use crate::gamestate::{GameState, Game};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_score)
            .insert_resource(ScoreTimer(Timer::from_seconds(3.2, TimerMode::Repeating)));
    }
}

#[derive(Resource)]
struct ScoreTimer(Timer);

fn add_score(mut score_timer: ResMut<ScoreTimer>, mut gamestate: ResMut<GameState>, time: Res<Time>) {
    if gamestate.0.gamestate != Game::Running {
        return;
    }
    score_timer.0.tick(time.delta());
    if score_timer.0.finished() {
        gamestate.0.score += 1;
    }
}
