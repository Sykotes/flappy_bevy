mod background;
mod bird;
mod pipes;
mod gamestate;

use bevy::{prelude::*, window::WindowResolution};

use background::BackgroundPlugin;
use bird::BirdPlugin;
use pipes::PipesPlugin;
use gamestate::{GameState, Game, GameData};

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
        .add_plugins(BirdPlugin)
        .add_plugins(PipesPlugin)
        .add_plugins(BackgroundPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_gamestates)
        .run();
}

fn setup(mut commands: Commands) {
    commands.insert_resource(GameState(GameData { gamestate: Game::Opened, score: 0 } ));
    commands.spawn(Camera2dBundle::default());
}

fn handle_gamestates(
    input_keys: Res<Input<KeyCode>>,
    mut gamestate: ResMut<GameState>
) {
    if gamestate.0.gamestate == Game::Opened && input_keys.just_pressed(KeyCode::Space) {
        gamestate.0.gamestate = Game::Running;
    }
    if gamestate.0.gamestate == Game::Over {
        println!("Score: {}", gamestate.0.score);
    }
}
