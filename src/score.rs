use bevy::prelude::*;

use crate::gamestate::{GameState, Game};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_score)
            .add_systems(Update, print_score)
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

struct PrintedScore(bool);

impl Default for PrintedScore {
    fn default() -> Self {
        PrintedScore(false)
    }
}

fn print_score(
    gamestate: Res<GameState>,
    mut printed_score: Local<PrintedScore>,
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    if gamestate.0.gamestate == Game::Over {
        if !printed_score.0 {
            commands.spawn(SpriteBundle{
                texture: asset_server.load("score_text.png"),
                transform: Transform { translation: Vec3::new(0.0, 80.0, 100.0), scale: Vec3::new(2.0, 2.0, 2.0) , ..default()},
                ..default()
            });
            let score_string = gamestate.0.score.to_string();
            let score_string_len = score_string.len();
            let num_size = 64.0;
            let score_len = num_size * score_string_len as f32;
            let start_pos = 0.0 - score_len / 2.0 + num_size / 2.0;
            let mut i: f32 = 0.0;
            for c in score_string.chars() {
                let path_string = format!("{}.png", c);
                commands.spawn(SpriteBundle{
                    texture: asset_server.load(path_string),
                    transform: Transform { translation: Vec3::new(start_pos + i * num_size, 0.0, 100.0), scale: Vec3::new(2.0, 2.0, 2.0) , ..default()},
                    ..default()
                });
                i += 1.0;
            }
            printed_score.0 = true;
        }
    }
}
