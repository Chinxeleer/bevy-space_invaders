use bevy::{prelude::*, window::PrimaryWindow};

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship)
            .add_systems(Update, space_ship_movement_system);
    }
}

#[derive(Component)]
pub struct SpaceShip;

pub const SHIP_SPEED: f32 = 500.0;
pub const LEFT_WALL: f32 = 0.0;
pub const RIGHT_WALL: f32 = 1280.0;
pub const SPACESHIP_SIZE: f32 = 28.0;

fn spawn_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, 30.0, 1.0)
                .with_scale(Vec3::splat(0.5)),
            texture: asset_server.load("sprites/ship/space_ship.png"),
            ..Default::default()
        },
        SpaceShip,
    ));
}

fn space_ship_movement_system(
    mut query: Query<&mut Transform, With<SpaceShip>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut ship = query.single_mut();
    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let new_ship_position = ship.translation.x + direction * SHIP_SPEED * time.delta_seconds();

    let left_bound = LEFT_WALL + SPACESHIP_SIZE / 2.0 + 10.0;

    let right_bound = RIGHT_WALL - SPACESHIP_SIZE / 2.0 - 10.0;

    ship.translation.x = new_ship_position.clamp(left_bound, right_bound);
}
