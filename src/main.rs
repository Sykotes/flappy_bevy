mod background;
mod bird;
mod pipes;
mod collision_detection;

use bevy::{prelude::*, window::WindowResolution};

use background::BackgroundPlugin;
use bird::BirdPlugin;
use pipes::PipesPlugin;
use collision_detection::CollisionDetectionPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bevy".into(),
                        resolution: WindowResolution::new(432.0, 768.0),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_plugins(BirdPlugin)
        .add_plugins(PipesPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
