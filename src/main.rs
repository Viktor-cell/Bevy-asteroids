mod player;
mod components;
mod bullet;
mod asteroid;
mod colisions;
mod labels;

use bevy::{prelude::*, window::PrimaryWindow};
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
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
                bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            )
        )
        .add_plugins(player::PlayerPlugin)
        //.add_plugins(bullet::BulletPlugin)
        //.add_plugins(asteroid::AsteroidsPlugin)
        //.add_plugins(colisions::ColisionsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(PostStartup, setup)
        .run();
}

#[derive(Resource)]
struct GameWindow(Vec2);

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
