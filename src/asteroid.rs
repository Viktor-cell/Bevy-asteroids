use bevy::{
    color::palettes::css::WHITE, prelude::*
};

use crate::{components::*, GameWindow};

const MAX_NUMBER: usize = 1;
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
) {
    if asteroids_info.count >= MAX_NUMBER {
        return;
    }
    let asteroid = meshes.add(Mesh::from(RegularPolygon::new(25., 7)));
    let material_bundle = materials.add(ColorMaterial::from_color(WHITE));

    c.spawn(Mesh2d(asteroid))
        .insert(MeshMaterial2d(material_bundle))
        .insert(Transform::default())
        .insert(Velocity::new(150., 150.))
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
    asteroids_transform.iter_mut().for_each(|mut asteroid_transform| {

        if asteroid_transform.translation.x > window.0.x / 2. {
            asteroid_transform.translation.x -= window.0.x;
        }

        if asteroid_transform.translation.x < -(window.0.x / 2.) {
            asteroid_transform.translation.x += window.0.x;
        }

        if asteroid_transform.translation.y > window.0.y / 2. {
            asteroid_transform.translation.y -= window.0.y;
        }

        if asteroid_transform.translation.y < -(window.0.y / 2.) {
            asteroid_transform.translation.y += window.0.y;
        }
    });
}
