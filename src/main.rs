use bevy::{prelude::*, window::WindowResolution};

pub mod bird;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bevy".into(),
                        resolution: WindowResolution::new(960.0, 540.0),
                        //resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_plugins(bird::BirdPlugin)
        .run();
}

fn setup(mut commands: Commands, mut color: ResMut<ClearColor>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    *color = ClearColor(Color::BEIGE);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("background.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(5.0, 5.0, 0.0),
                ..default()
            },
            ..default()
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ground.png"),
            transform: Transform {
                scale: Vec3::new(5.0, 5.0, 0.0),
                translation: Vec3::new(0.0, -232.0, 1.0),
                ..default()
            },
            ..default()
        },
    ));
}
