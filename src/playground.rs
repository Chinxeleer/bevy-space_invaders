use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::resources::Score;

pub struct SettingsPlugin;

#[derive(Component)]
pub struct ScoreText;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_scoreboard))
            .add_systems(Update, (exit_game, update_font));
    }
}

fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

fn exit_game(mut exit_game: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit_game.send(AppExit);
    }
}

fn setup_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // load font from asset folder
    let font = asset_server.load("fonts/firaCode.ttf");
    let window = window_query.get_single().unwrap();

    // text style configuration
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 70.0,
        color: Color::WHITE,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Score :", text_style.clone()),
                TextSection::from_style(text_style.clone()),
            ]),
            transform: Transform::from_xyz(120.0, window.height() - 40.00, 0.0),
            ..Default::default()
        },
        ScoreText,
    ));
}

fn update_font(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    for mut text in query.iter_mut() {
        text.sections[1].value = score.score.to_string();
    }
}
