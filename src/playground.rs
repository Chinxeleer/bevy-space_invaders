use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}
//
// fn ship_boundary_line(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(MaterialMesh2dBundle {
//         mesh: meshes.add(Rectangle::default()).into(),
//         transform: Transform::from_xyz(600., 0., 0.),
//         material: materials.add(Color::PURPLE),
//         ..Default::default()
//     });
// }
