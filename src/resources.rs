use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BulletCacheSpawnTimer>()
            .init_resource::<BulletCache>();
    }
}
pub const BULLET_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct BulletCache {
    pub total_bullets: u8,
    pub cache: i32,
}

impl Default for BulletCacheSpawnTimer {
    fn default() -> Self {
        Self {
            time: Timer::from_seconds(BULLET_SPAWN_TIME, TimerMode::Once),
        }
    }
}

// TODO: Make the bullet cache lag in reloading the bullets again.

#[derive(Resource)]
struct BulletCacheSpawnTimer {
    time: Timer,
}

impl Default for BulletCache {
    fn default() -> Self {
        BulletCache {
            total_bullets: 5,
            cache: 0,
        }
    }
}
