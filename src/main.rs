use bevy::prelude::*;
use space_invaders::aliens::AlienPlugin;
use space_invaders::bullet::BulletPlugin;
use space_invaders::playground::SettingsPlugin;
use space_invaders::resources::ResourcesPlugin;
use space_invaders::ship_plugin::ShipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SettingsPlugin)
        .add_plugins(ShipPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(AlienPlugin)
        .add_plugins(ResourcesPlugin)
        .run();
}
