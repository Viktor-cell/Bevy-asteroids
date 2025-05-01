mod player;
mod components;
mod bullet;
mod asteroid;
mod colisions;

use bevy::prelude::*;
use rand::{rngs::SmallRng, SeedableRng};

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800.0, 600.0).into(),
                        resizable: false,
                        present_mode: bevy::window::PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                }), 
                bevy::diagnostic::LogDiagnosticsPlugin::default(),
                bevy::diagnostic::FrameTimeDiagnosticsPlugin::default()
            )
        )
        .add_plugins(player::PlayerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(asteroid::AsteroidsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        //    .add_systems(Update, print_fps_system)
        .run();
}

#[derive(Resource)]
struct GameWindow(Vec2);

#[derive(Resource)]
struct Random(rand::rngs::SmallRng);

fn setup(
    mut c: Commands
) {
    c.insert_resource(GameWindow(Vec2::new(800., 600.)));
    c.insert_resource(asteroid::AsteroidsInfo::default());
    c.insert_resource(Random(SmallRng::from_os_rng()));
}
