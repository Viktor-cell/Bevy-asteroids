use bevy::prelude::*;

#[derive(Component,)]
pub struct Player;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component, Clone, Copy)]
pub struct Health(pub i32);

impl Health {
    pub fn from_collider(collider: &Collider) -> Self {
        Self((collider.radius / 10.).round() as i32)
    }
}

#[derive(Component, Clone, Copy)]
pub struct Collider {
    pub radius: f32
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity(Vec2 { x, y })
    }
}

#[derive(Component)]
pub struct LabelFollows(pub Entity);
