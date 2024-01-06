use bevy::prelude::*;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
        app.add_systems(Update, flap);
        app.add_systems(Update, fly);
    }
}

#[derive(Component)]
struct Bird;

fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bird_layer1.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(4.0, 4.0, 0.0),
                ..default()
            },
            ..default()
        },
        Bird,
    ));
}

fn flap(
    mut bird: Query<(&mut Handle<TextureAtlas>, With<Bird>)>,
    input_keys: Res<Input<KeyCode>>,
    input_mouse: Res<Input<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (mut texture, _) in &mut bird {
        if input_keys.just_pressed(KeyCode::Space) || input_mouse.just_pressed(MouseButton::Left) {
            let texture_handle = asset_server.load("bird_layer2.png");
            let texture_atlas =
                TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 1, 1, None, None);
            *texture = texture_atlases.add(texture_atlas);
        }
    }
}

struct Velocity(f32);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(0.0)
    }
}

fn fly(
    mut bird: Query<(&mut Transform, With<Bird>)>,
    input_keys: Res<Input<KeyCode>>,
    input_mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut velocity: Local<Velocity>,
) {
    for (mut transform, _) in &mut bird {

        velocity.0 -= 1000.0 * time.delta_seconds();
        if velocity.0 < -400.0 {
            velocity.0 = -400.0;
        }

        if input_keys.just_pressed(KeyCode::Space) || input_mouse.just_pressed(MouseButton::Left) {
            velocity.0 += 500.0;
            if velocity.0 > 350.0 {
                velocity.0 = 350.0;
            }
        }

        transform.translation.y += velocity.0 * time.delta_seconds();
    }
}
