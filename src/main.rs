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
                ..default()
            }),
            ..default()
        }))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
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
