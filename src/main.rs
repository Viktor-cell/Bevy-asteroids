mod player;
mod components;
mod bullet;
mod asteroid;
mod colisions;
mod labels;

use bevy::{prelude::*, window::PrimaryWindow};
use components::Asteroid;
use rand::{
    rngs::SmallRng,
    SeedableRng
};

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800., 600.).into(),
                        resizable: false,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }), 
                //bevy::diagnostic::LogDiagnosticsPlugin::default(),
                //bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            )
        )
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Score(0))
        .insert_state(GameState::Start)
        .add_systems(Startup, setup)
        .add_plugins((
            player::PlayerPlugin,
            bullet::BulletPlugin,
            asteroid::AsteroidsPlugin,
            colisions::ColisionsPlugin,
            UI
        ))
        .run();
}

#[derive(Resource)]
struct GameWindow(Vec2);

#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct Random(rand::rngs::SmallRng);

fn setup(
    mut c: Commands,
    window: Single<&Window, With<PrimaryWindow>>
) {
    let window = window.into_inner();
    c.insert_resource(GameWindow(Vec2::new(window.width(), window.height())));
    c.insert_resource(asteroid::AsteroidsInfo::default());
    c.insert_resource(Random(SmallRng::from_os_rng()));
    c.spawn(Camera2d);
}

#[derive(States, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
enum GameState {
    Start,
    Playing,
    End
}

struct UI;

impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), (
            start_screen,
            hide_score,
        ))
        .add_systems(OnEnter(GameState::Playing), reset_score)
        .add_systems(Update, start_game_system.run_if(in_state(GameState::Start).and(should_start_game)))
        .add_systems(OnEnter(GameState::End), restart_screen)
        .add_systems(Update, start_game_system.run_if(in_state(GameState::End).and(should_restart_game)))
        .add_systems(Update, show_score.run_if(in_state(GameState::Playing)));
    }
}

fn start_screen(
    mut c: Commands,
) {
    c.spawn(
        (
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Button,
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Text::new("Press to start"),
                            TextColor(Color::WHITE)
                        )
                    ]
                )
            ]
        )
    );
}

fn start_game_system(
    mut c : Commands,
    button: Query<Entity, With<Button>>,
    asteroids_entities: Query<Entity, With<Asteroid>>,
) -> Result {
    for asteroid_entity in asteroids_entities.iter() {
        c.entity(asteroid_entity).despawn();
    }
    c.set_state(GameState::Playing);
    if let Ok(entity) = button.single() {
        c.entity(entity).despawn();
    }
    Ok(())
}

fn should_start_game(
    button_interaction: Query<&Interaction, With<Button>>
) -> bool {
    *button_interaction.single().unwrap() == Interaction::Pressed
}

fn restart_screen(
    mut c : Commands,
) {
    c.spawn(
        (
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![
                (
                    Button,
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Text::new("Restart"),
                            TextColor(Color::WHITE)
                        )
                    ]
                )
            ]
        )
    );
}

fn should_restart_game(
    button_interaction: Query<&Interaction, With<Button>>
) -> bool {
    *button_interaction.single().unwrap() == Interaction::Pressed
}

#[derive(Component)]
struct ScoreLabel;

fn hide_score(
    mut c : Commands,
    score_label: Query<Entity, With<ScoreLabel>>
) {
    if let Ok(score_label) = score_label.single() {
        c.entity(score_label).despawn();
    }
}

fn reset_score(
    mut score: ResMut<Score>,
) {
    score.0 = 0;
}

fn show_score(
    mut c : Commands,
    score: Res<Score>,
    score_label: Query<Entity, With<ScoreLabel>>
) {
    if let Ok(score_label) = score_label.single() {
        c.entity(score_label).despawn();
    }
    c.spawn(
        (
            ScoreLabel,
            Node {
                left: Val::Px(20.),
                top: Val::Px(20.),
                ..default()
            },
            children![
                (
                    Text::new(format!("Score: {}", score.0)),
                    TextColor(Color::WHITE)
                ) 
            ]
        )
    );
}
