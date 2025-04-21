mod player;
mod components;
mod bullet;

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
        .run();
}

