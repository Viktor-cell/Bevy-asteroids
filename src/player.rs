use bevy::{
    prelude::*,
    render::mesh::shape::RegularPolygon,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, init_player_system)
        .add_systems(Update, move_player_system)
        .add_systems(Update, rotate_player_system)
        .add_systems(Update, wrap_player_system)
        .add_systems(Update, shoot_bullet_system);
    }
}

use std::f32::consts::PI;
use crate::components::*;

pub fn init_player_system(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    c.spawn(Camera2dBundle::default());

    let mesh_handle = meshes.add(Mesh::from(RegularPolygon::new(10., 3)));
    let material_handle = materials.add(Color::WHITE.into());

    c.spawn(
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_handle),
            material: material_handle,
            ..default()
        },
    )
        .insert(Player)
        .insert(Velocity::default());
}

const ACCELERATION: f32 = 300.;
const MAX_SPEED: f32 = 500.;
const DRAG: f32 = 300.;

pub fn move_player_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
    mut player_velocity: Query<&mut Velocity, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut player_transform = player_transform.single_mut();
    let mut player_velocity = player_velocity.single_mut();

    let forward_dir = player_transform.rotation * Vec3::Y;

    if keyboard.pressed(KeyCode::W) && player_velocity.0.length() <= MAX_SPEED {
        player_velocity.0 += ACCELERATION * time.delta_seconds();
    } else {
        player_velocity.0.x = (player_velocity.0.x - DRAG * time.delta_seconds()).max(0.);
        player_velocity.0.y = (player_velocity.0.y - DRAG * time.delta_seconds()).max(0.);
    }

    player_transform.translation += forward_dir * player_velocity.0.extend(0.) * time.delta_seconds();
}

pub fn rotate_player_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();
    let mut player_transform = player_transform.single_mut();

    if let Some(cursor_pos) = window.cursor_position() {

        let cursor_pos = Vec2::new(
            cursor_pos.x - window.width() / 2.,
            -(cursor_pos.y - window.height() / 2.),
        );

        if cursor_pos.distance(player_transform.translation.truncate()) < 4. {
            return;
        }

        let direction = cursor_pos - player_transform.translation.truncate();
        let angle = direction.y.atan2(direction.x) - PI / 2.;

        player_transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn wrap_player_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>
) {
    let mut player_transform = player_transform.single_mut();
    let window = window.single();

    if player_transform.translation.x > window.width() / 2. {
        player_transform.translation.x -= window.width();
    }

    if player_transform.translation.x < -(window.width() / 2.) {
        player_transform.translation.x += window.width();
    }

    if player_transform.translation.y > window.height() / 2. {
        player_transform.translation.y -= window.height();
    }

    if player_transform.translation.y < -(window.height() / 2.) {
        player_transform.translation.y += window.height();
    }

}

pub fn shoot_bullet_system(
    player_transform: Query<&Transform, With<Player>>,
    c: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        crate::bullet::spawn_bullet_system(player_transform, c, meshes, materials);
    }
}
