use crate::resources::{BulletCache, BulletCacheSpawnTimer, Score};
use super::aliens::AlienMarker;
use super::ship_plugin as Ship;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_bullet,
                bullet_movement_system,
                despawn_bullets,
                collision_with_alien,
            ),
        );
    }
}

const BULLET_SPEED: f32 = 400.0;

#[derive(Component)]
struct Bullet;

// function to spawn bullets and keep track of the bullets in the cache.
fn spawn_bullet(
    mut bullet_cache: ResMut<BulletCache>,
    mut commands: Commands,
    time: ResMut<Time>,
    mut spawn_timer: ResMut<BulletCacheSpawnTimer>,
    query: Query<&Transform, With<Ship::SpaceShip>>,
    asset_server: Res<AssetServer>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let space_ship_position = query.get_single().unwrap().translation;

    if (bullet_cache.cache < (bullet_cache.total_bullets) as i32)
        && keyboard.just_pressed(KeyCode::Space)
    {
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
        bullet_cache.cache += 1;
    }

    // checking if we still have bullets in the cash. If not, we start the timer.
    if bullet_cache.cache == bullet_cache.total_bullets as i32 {
        spawn_timer.timer.tick(time.delta());
    }

    // The time has started ticking. Now we check if timer has reached the end.
    if spawn_timer.timer.finished() {
        bullet_cache.cache = 0;
        spawn_timer.timer.reset();
    }

    println!("{}", bullet_cache.cache);
}

fn despawn_bullets(
    // mut bullet_cache: ResMut<BulletCache>,
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let upper_bound = window.height();
    for (entity, transform) in query.iter() {
        if transform.translation.y >= upper_bound {
            commands.entity(entity).despawn();
            // bullet_cache.cache -= 1
        }
    }
}

fn bullet_movement_system(mut query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.y += BULLET_SPEED * time.delta_seconds();
    }
}

fn collision_with_alien(
    // mut bullet_cache: ResMut<BulletCache>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    alien_query: Query<(Entity, &Transform), With<AlienMarker>>,
) {
    for (bullet_entity, bullet_query) in &bullet_query {
        for (alien_entity, alien_query) in &alien_query {
            let distance =
                Vec3::new(bullet_query.translation.x, bullet_query.translation.y, 0.0).distance(
                    Vec3::new(alien_query.translation.x, alien_query.translation.y, 0.0),
                );

            if distance <= 50.0 {
                commands.entity(alien_entity).despawn();
                commands.entity(bullet_entity).despawn();
                score.score += 1;
                // bullet_cache.cache -= 1
            }
        }
    }
}
