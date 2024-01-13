use bevy::prelude::*;
use rand::prelude::*;

use crate::gamestate::{GameState, Game};

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_pipes)
            .add_systems(Update, move_pipes)
            .add_systems(Update, delete_pipes)
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(
                3.2,
                TimerMode::Repeating,
            )));
    }
}

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Component)]
struct Pipe;

fn spawn_pipes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut pipe_spawn_timer: ResMut<PipeSpawnTimer>,
    time: Res<Time>,
    gamestate: Res<GameState>,
) {
    if gamestate.0.gamestate != Game::Running {
        return;
    }
    pipe_spawn_timer.0.tick(time.delta());
    if pipe_spawn_timer.0.finished() {
        let texture_handle = asset_server.load("pipe.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 128.0), 1, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let mut rng = rand::thread_rng();
        let pipe_offset: f64 = rng.gen_range(-100.0..=100.0) - 180.0;
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone_weak(),
                transform: Transform {
                    translation: Vec3::new(250.0, pipe_offset as f32, 5.0),
                    scale: Vec3::new(3.0, 3.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Pipe,
        ));
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(250.0, pipe_offset as f32 + 480.0, 5.0),
                    scale: Vec3::new(3.0, 3.0, 0.0),
                    rotation: Quat::from_rotation_x(3.141592654),
                    ..default()
                },
                ..default()
            },
            Pipe,
        ));
    }
}

fn move_pipes(mut pipe: Query<(&mut Transform, With<Pipe>)>, time: Res<Time>) {
    for (mut transform, _) in &mut pipe {
        transform.translation.x -= 110.0 * time.delta_seconds();
    }
}

fn delete_pipes(mut commands: Commands, mut pipe: Query<(Entity, &mut Transform), With<Pipe>>) {
    for (entity, transform) in &mut pipe {
        if transform.translation.x < -300.0 {
            commands.entity(entity).despawn();
        }
    }
}
