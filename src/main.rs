use bevy::prelude::*;

pub mod bird;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bevy".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_plugins(bird::BirdPlugin)
        .run();
}

fn setup(mut commands: Commands, mut color: ResMut<ClearColor>) {
    commands.spawn(Camera2dBundle::default());
    *color = ClearColor(Color::BEIGE);
}
