use bevy::prelude::*;

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird)
            .add_systems(Update, flap)
            .add_systems(Update, fly)
            .insert_resource(WingFlapTimer(Timer::from_seconds(
                0.2,
                TimerMode::Repeating,
            )));
    }
}

#[derive(Resource)]
struct WingFlapTimer(Timer);

#[derive(Component)]
struct Bird;

fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bird.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-150.0, 200.0, 50.0),
                scale: Vec3::new(3.0, 3.0, 0.0),
                ..default()
            },
            ..default()
        },
        Bird,
    ));
}

fn flap(mut bird: Query<(&mut TextureAtlasSprite, With<Bird>)>, input_keys: Res<Input<KeyCode>>) {
    for (mut texture_atas_sprite, _) in &mut bird {
        if input_keys.just_pressed(KeyCode::Space) {
            texture_atas_sprite.index = 1;
        }
        if input_keys.just_released(KeyCode::Space) {
            texture_atas_sprite.index = 0;
        }
    }
}

struct Velocity(f32);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(0.0)
    }
}

struct Angle(f32);

impl Default for Angle {
    fn default() -> Self {
        Angle(0.0)
    }
}

fn fly(
    mut bird: Query<(&mut Transform, With<Bird>)>,
    input_keys: Res<Input<KeyCode>>,
    input_mouse: Res<Input<MouseButton>>,
    time: Res<Time>,
    mut velocity: Local<Velocity>,
    mut angle: Local<Angle>,
) {
    for (mut transform, _) in &mut bird {
        const MAX_VELO: f32 = 240.0;
        const MIN_VELO: f32 = -300.0;
        const FLAP_STENGTH: f32 = 350.0; // max possible is MAX_VELO - MIN_VELO
        const GRAVITY: f32 = 700.0;

        velocity.0 -= GRAVITY * time.delta_seconds();
        if velocity.0 < MIN_VELO {
            velocity.0 = MIN_VELO;
        }

        if input_keys.just_pressed(KeyCode::Space) || input_mouse.just_pressed(MouseButton::Left) {
            velocity.0 += FLAP_STENGTH;
            if velocity.0 > MAX_VELO {
                velocity.0 = MAX_VELO;
            }
        }

        transform.translation.y += velocity.0 * time.delta_seconds();

        angle.0 += velocity.0 * 0.006 * time.delta_seconds();
        if angle.0 > 0.5 {
            angle.0 = 0.5;
        }
        if angle.0 < -0.5 {
            angle.0 = -0.5;
        }
        transform.rotation = Quat::from_rotation_z(angle.0)
    }
}
