use bevy::{
    color::palettes::css::WHITE, prelude::*
};
use rand::Rng;
use std::f32::consts::PI;

use crate::{components::*, GameWindow, Random};

const MAX_NUMBER: usize = 20;
const SPAWN_COOLDOWN: f32 = 2.;

#[derive(Resource)]
pub struct AsteroidsInfo{
    pub count: usize,
    pub timer_to_next: Timer,
}

impl Default for AsteroidsInfo {
    fn default() -> Self {
        Self {
            count: 0,
            timer_to_next: Timer::from_seconds(SPAWN_COOLDOWN, TimerMode::Repeating)
        }
    }
}

pub struct  AsteroidsPlugin;
impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_asteroid.run_if(asteroid_should_spawn))
            .add_systems(Update, move_asteroids_system)
            .add_systems(Update, wrap_asteroids_system);
    }
}

pub fn spawn_asteroid(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asteroids_info: ResMut<AsteroidsInfo>,
    window: Res<GameWindow>,
    mut rnd: ResMut<Random>
) {
    if asteroids_info.count >= MAX_NUMBER {
        return;
    }

    let asteroid = meshes.add(Mesh::from(RegularPolygon::new(25., 7)));
    let material_bundle = materials.add(ColorMaterial::from_color(WHITE));

    let mut spawn_pos = || -> Vec2 {
        let half_window_height = window.0.y / 2.;
        let half_window_width = window.0.x / 2.;
        let padding = 0.;

        match rnd.0.random_range(0..4) {
            0 => Vec2::new(-padding - half_window_width , rnd.0.random_range(-half_window_height..half_window_height)),
            1 => Vec2::new(padding + half_window_width, rnd.0.random_range(-half_window_height..half_window_height)),
            2 => Vec2::new( rnd.0.random_range(-half_window_width..half_window_width), -padding - half_window_height),
            3 => Vec2::new( rnd.0.random_range(-half_window_width..half_window_width), padding + half_window_height),
            _ => unreachable!()
        }
    };

    let transforn = Transform {
        translation: spawn_pos().extend(0.),
        rotation: Quat::from_rotation_z(rnd.0.random_range(-PI..PI)),
        scale: Vec2::splat(rnd.0.random_range(0.5..1.5)).extend(0.)
    };

    let velocity = Velocity::new(rnd.0.random_range(-150.0..150.0), rnd.0.random_range(-150.0..150.0));

    c.spawn(Mesh2d(asteroid))
        .insert(MeshMaterial2d(material_bundle))
        .insert(transforn)
        .insert(velocity)
        .insert(Asteroid);
    asteroids_info.count += 1;
}

pub fn asteroid_should_spawn(
    asteroids_info: Res<AsteroidsInfo>
) -> bool {
    asteroids_info.timer_to_next.finished()
}

pub fn move_asteroids_system (
    mut asteroids_info: ResMut<AsteroidsInfo>,
    time: Res<Time>,
    mut asteroinds: Query<(&mut Transform, &Velocity), With<Asteroid>>
) {
    asteroids_info.timer_to_next.tick(time.delta());

    asteroinds.iter_mut().for_each(|(mut asteroid_transform, asteroid_velocity)| {
        asteroid_transform.translation += asteroid_velocity.0.extend(0.) * time.delta_secs();
    });
}

pub fn wrap_asteroids_system(
    window: Res<GameWindow>,
    mut asteroids_transform: Query<&mut Transform, With<Asteroid>>
) {
    let half_window_height = window.0.y / 2.;
    let half_window_width = window.0.x / 2.;

    asteroids_transform.iter_mut().for_each(|mut asteroid_transform| {
        if asteroid_transform.translation.x > half_window_width {
            asteroid_transform.translation.x -= window.0.x;
        }

        if asteroid_transform.translation.x < -half_window_width {
            asteroid_transform.translation.x += window.0.x;
        }

        if asteroid_transform.translation.y > half_window_height {
            asteroid_transform.translation.y -= window.0.y;
        }

        if asteroid_transform.translation.y < -half_window_height {
            asteroid_transform.translation.y += window.0.y;
        }
    });
}
