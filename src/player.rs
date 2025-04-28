use bevy::{
    color::palettes::css::WHITE, prelude::*, window::PrimaryWindow
};

use std::f32::consts::PI;
use crate::{components::*, GameWindow};
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

const ACCELERATION: f32 = 450.;
const MAX_SPEED: f32 = 600.;
const DRAG: f32 = 300.;

pub fn init_player_system(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    c.spawn(Camera2d::default());

    let mesh_handle = meshes.add(Mesh::from(RegularPolygon::new(10., 3)));
    let material_handle = materials.add(ColorMaterial::from_color(WHITE));

    c.spawn(Mesh2d(mesh_handle))
        .insert(MeshMaterial2d(material_handle))
        .insert(Transform::default())
        .insert(Velocity::default())
        .insert(Player);
}


pub fn move_player_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
    mut player_velocity: Query<&mut Velocity, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) -> Result {
    let mut player_transform = player_transform.single_mut()?;
    let mut player_velocity = player_velocity.single_mut()?;

    let forward_dir = (player_transform.rotation * Vec3::Y).normalize();


    if keyboard.pressed(KeyCode::KeyW) {
        let next_velocity = Vec2 {
            x: player_velocity.0.x + forward_dir.x * ACCELERATION * time.delta_secs(),
            y: player_velocity.0.y + forward_dir.y * ACCELERATION * time.delta_secs(),
        };
        if next_velocity.length() <= MAX_SPEED {
            player_velocity.0 = next_velocity;
        }
    }else if player_velocity.0.length().abs() > 0. {
        player_velocity.0 = player_velocity.0 - player_velocity.0.normalize() * DRAG * time.delta_secs();
    }

    player_transform.translation += player_velocity.0.extend(0.) * time.delta_secs();

    Ok(())
}

pub fn rotate_player_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
    window: Query<&Window, With<PrimaryWindow>>,
) -> Result {
    let window = window.single()?;
    let mut player_transform = player_transform.single_mut()?;

    if let Some(cursor_pos) = window.cursor_position() {

        let cursor_pos = Vec2::new(
            cursor_pos.x - window.width() / 2.,
            -(cursor_pos.y - window.height() / 2.),
        );

        if cursor_pos.distance(player_transform.translation.truncate()) < 4. {
            return Ok(());
        }

        let direction = cursor_pos - player_transform.translation.truncate();
        let angle = direction.y.atan2(direction.x) - PI / 2.;

        player_transform.rotation = Quat::from_rotation_z(angle);
    }

    Ok(())
}

pub fn wrap_player_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
    window: Res<GameWindow>
) -> Result {
    let mut player_transform = player_transform.single_mut()?;

    if player_transform.translation.x > window.0.x / 2. {
        player_transform.translation.x -= window.0.x;
    }

    if player_transform.translation.x < -(window.0.x / 2.) {
        player_transform.translation.x += window.0.x;
    }

    if player_transform.translation.y > window.0.y / 2. {
        player_transform.translation.y -= window.0.y;
    }

    if player_transform.translation.y < -(window.0.y / 2.) {
        player_transform.translation.y += window.0.y;
    }
    
    Ok(())
}

pub fn shoot_bullet_system(
    player_transform: Query<&Transform, With<Player>>,
    c: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) -> Result {
    let player_transform = player_transform.single()?;
    if keyboard.just_pressed(KeyCode::Space) {
        crate::bullet::spawn_bullet_system(player_transform, c, meshes, materials);
    }

    Ok(())
}
