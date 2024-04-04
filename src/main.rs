use bevy::prelude::*;
use space_invaders::aliens::AlienPlugin;
use space_invaders::bullet::BulletPlugin;
use space_invaders::playground as Screen;
use space_invaders::ship_plugin as Ship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Screen::ScreenPlugin)
        .add_plugins(Ship::ShipPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(AlienPlugin)
        .run();
}
