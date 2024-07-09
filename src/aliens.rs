use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;
pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_alien, alien_movement))
            .init_resource::<AlienSpawnTimer>();
    }
}

const ALIEN_SPEED: f32 = 400.0;
const ALIEN_SPAWN_TIME: f32 = 1.0;
const ALIEN_SIZE: f32 = 50.0;

#[derive(Component)]
pub struct AlienMarker;

#[derive(Resource)]
struct AlienSpawnTimer {
    time: Timer,
}

impl Default for AlienSpawnTimer {
    fn default() -> Self {
        Self {
            time: Timer::from_seconds(ALIEN_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

// fn spawn_alien(
//     mut timer: ResMut<AlienSpawnTimer>,
//     mut commands: Commands,
//     assert_server: Res<AssetServer>,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     time: Res<Time>,
// ) {
//     let window = window_query.get_single().unwrap();
//
//     timer.time.tick(time.delta());
//     let random = rand::thread_rng().gen_range(0..=5);
//     if timer.time.finished() {
//         for _ in 0..random {
//             let random_x = rand::random::<f32>() * window.width() + 2.0 * ALIEN_SIZE;
//
//             if random_x > window.width() - ALIEN_SIZE {
//                 continue;
//             }
//             commands.spawn((
//                 SpriteBundle {
//                     transform: Transform::from_xyz(random_x, window.height(), 0.0)
//                         .with_scale(Vec3::splat(0.5)),
//                     texture: assert_server.load("sprites/aliens/shipBeige_manned.png"),
//                     ..Default::default()
//                 },
//                 AlienMarker,
//             ));
//         }
//     }
// }
//
//
//
fn spawn_alien(
    mut timer: ResMut<AlienSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();

    timer.time.tick(time.delta());
    if timer.time.finished() {
        let mut rng = rand::thread_rng();
        let random_count = rng.gen_range(1..=5);

        // Calculate the step for even distribution
        let step = window.width() / (random_count + 1) as f32;

        // Generate positions without overlap
        let mut positions = Vec::new();
        for i in 1..=random_count {
            let x_pos = step * i as f32;
            positions.push(x_pos);
        }

        for x in positions {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(x, window.height(), 0.0)
                        .with_scale(Vec3::splat(0.5)),
                    texture: asset_server.load("sprites/aliens/shipBeige_manned.png"),
                    ..Default::default()
                },
                AlienMarker,
            ));
        }
    }
}

fn alien_movement(mut alien: Query<&mut Transform, With<AlienMarker>>, time: Res<Time>) {
    for mut transform in alien.iter_mut() {
        transform.translation.y -= ALIEN_SPEED * time.delta_seconds();
    }
}
