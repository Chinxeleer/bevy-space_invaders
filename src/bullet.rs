use super::ship_plugin as Ship;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_bullet, bullet_movement_system, despawn_bullets),
        );
    }
}

const BULLET_SPEED: f32 = 400.0;

#[derive(Component)]
struct Bullet;

fn spawn_bullet(
    mut commands: Commands,
    query: Query<&Transform, With<Ship::SpaceShip>>,
    asset_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let space_ship_position = query.get_single().unwrap().translation;

    if keyboard.just_pressed(KeyCode::Space) {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    space_ship_position.x,
                    space_ship_position.y + 13.0,
                    0.0,
                )
                .with_scale(Vec3::splat(0.1)),
                texture: asset_server.load("sprites/ship/bullet.png"),
                ..default()
            },
            Bullet,
        ));
    }
}

fn despawn_bullets(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let upper_bound = window.height();
    for (entity, transform) in query.iter() {
        if transform.translation.y >= upper_bound {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_movement_system(mut query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.y += BULLET_SPEED * time.delta_seconds();
    }
}
