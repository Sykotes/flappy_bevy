use bevy::prelude::*;

pub struct PipesPlugin;

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pipes)
            .insert_resource(PipeSpawnTimer(Timer::from_seconds(
                0.2,
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
) {
    let texture_handle = asset_server.load("pipe.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 128.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, -50.0, 5.0),
                scale: Vec3::new(3.0, 3.0, 0.0),
                ..default()
            },
            ..default()
        },
        Pipe,
    ));
}
