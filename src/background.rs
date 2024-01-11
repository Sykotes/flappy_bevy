use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_background)
            .add_systems(Startup, init_ground)
            .add_systems(Update, spawn_background)
            .add_systems(Update, move_background_1)
            .add_systems(Update, move_background_2)
            .add_systems(Update, spawn_ground)
            .add_systems(Update, move_ground)
            .insert_resource(Layer1SpawnTimer(Timer::from_seconds(
                17.95,
                TimerMode::Repeating,
            )))
            .insert_resource(Layer2SpawnTimer(Timer::from_seconds(
                7.17,
                TimerMode::Repeating,
            )))
            .insert_resource(GroundSpawnTimer(Timer::from_seconds(
                3.89,
                TimerMode::Repeating,
            )));
    }
}

#[derive(Component)]
struct Background1;

#[derive(Component)]
struct Background2;

#[derive(Component)]
struct Ground;

fn init_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..=2 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("background_layer1.png"),
                transform: Transform {
                    scale: Vec3::new(4.0, 4.0, 0.0),
                    translation: Vec3::new(i as f32 * 432.0 - 432.0 / 2.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Background1,
        ));
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("background_layer2.png"),
                transform: Transform {
                    scale: Vec3::new(4.0, 4.0, 0.0),
                    translation: Vec3::new(i as f32 * 432.0 - 432.0 / 2.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Background2,
        ));
    }
}

fn init_ground(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..=2 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("ground.png"),
                transform: Transform {
                    translation: Vec3::new(i as f32 * 432.0 - 432.0 / 2.0, -336.0, 10.0),
                    scale: Vec3::new(4.0, 4.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Ground,
        ));
    }
}

#[derive(Resource)]
struct Layer1SpawnTimer(Timer);

#[derive(Resource)]
struct Layer2SpawnTimer(Timer);

fn spawn_background(
    mut layer1_timer: ResMut<Layer1SpawnTimer>,
    mut layer2_timer: ResMut<Layer2SpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    layer1_timer.0.tick(time.delta());
    layer2_timer.0.tick(time.delta());
    if layer1_timer.0.finished() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("background_layer1.png"),
                transform: Transform {
                    scale: Vec3::new(4.0, 4.0, 0.0),
                    translation: Vec3::new((2.0 * 432.0) - (432.0 / 2.0) - 1.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Background1,
        ));
    }
    if layer2_timer.0.finished() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("background_layer2.png"),
                transform: Transform {
                    scale: Vec3::new(4.0, 4.0, 0.0),
                    translation: Vec3::new((2.0 * 432.0) - (432.0 / 2.0) - 1.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Background2,
        ));
    }
}

fn move_background_1(
    mut background_1: Query<(&mut Transform, With<Background1>)>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut background_1 {
        transform.translation.x -= 24.0 * time.delta_seconds();
    }
}

fn move_background_2(
    mut background_2: Query<(&mut Transform, With<Background2>)>,
    time: Res<Time>,
) {
    for (mut transform, _) in &mut background_2 {
        transform.translation.x -= 60.0 * time.delta_seconds();
    }
}

#[derive(Resource)]
struct GroundSpawnTimer(Timer);

fn spawn_ground(
    mut ground_spawn_timer: ResMut<GroundSpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    ground_spawn_timer.0.tick(time.delta());
    if ground_spawn_timer.0.finished() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("ground.png"),
                transform: Transform {
                    translation: Vec3::new((2.0 * 432.0) - (432.0 / 2.0) - 1.0, -336.0, 10.0),
                    scale: Vec3::new(4.0, 4.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Ground,
        ));
    }
}

fn move_ground(mut ground: Query<(&mut Transform, With<Ground>)>, time: Res<Time>) {
    for (mut transform, _) in &mut ground {
        transform.translation.x -= 110.0 * time.delta_seconds();
    }
}
