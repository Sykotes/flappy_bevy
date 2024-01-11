use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background)
            .add_systems(Startup, spawn_ground);
    }
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Ground;

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(3.0, 3.0, 0.0),
                ..default()
            },
            ..default()
        },
        Background,
    ));
}

fn spawn_ground() {}
