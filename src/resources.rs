use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BulletCacheSpawnTimer>()
            .init_resource::<BulletCache>()
            .init_resource::<Score>();
    }
}
pub const BULLET_SPAWN_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct BulletCache {
    pub total_bullets: u8,
    pub cache: i32,
}

impl Default for BulletCache {
    fn default() -> Self {
        BulletCache {
            total_bullets: 10,
            cache: 0,
        }
    }
}

#[derive(Resource)]
pub struct BulletCacheSpawnTimer {
    pub timer: Timer,
}

impl Default for BulletCacheSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BULLET_SPAWN_TIME, TimerMode::Once),
        }
    }
}
// TODO: Add the score resource.
#[derive(Resource, Default)]
pub struct Score {
    pub score: u32,
}
