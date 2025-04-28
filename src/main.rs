mod player;
mod components;
mod bullet;
mod asteroid;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 600.0).into(),
                resizable: false,
                present_mode: bevy::window::PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(asteroid::AsteroidsPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, print_fps_system)
        .run();
}

#[derive(Resource)]
struct GameWindow(Vec2);

fn setup(
    mut c: Commands
) {
    c.insert_resource(GameWindow(Vec2::new(800., 600.)));
    c.insert_resource(asteroid::AsteroidsInfo::default());
}

/// The system that will track and print the FPS
fn print_fps_system(time: Res<Time>) {
    // Calculate FPS by taking the inverse of the delta time

    let fps = 1.0 / time.delta_secs();
    println!("FPS: {:.2}", fps);  // Print FPS rounded to 2 decimal places
}

