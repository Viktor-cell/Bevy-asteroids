use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);


impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity(Vec2 { x, y })
    }
}
