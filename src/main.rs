use bevy::prelude::*;
use bevy::window::WindowResolution;
use space_invaders::aliens::AlienPlugin;
use space_invaders::bullet::BulletPlugin;
use space_invaders::playground::SettingsPlugin;
use space_invaders::resources::ResourcesPlugin;
use space_invaders::ship_plugin::ShipPlugin;

const GAME_WIDTH: f32 = 800.0;
const GAME_HEIGHT: f32 = 600.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(GAME_WIDTH, GAME_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(SettingsPlugin)
        .add_plugins(ShipPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(AlienPlugin)
        .add_plugins(ResourcesPlugin)
        .run();
}
