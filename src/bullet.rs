use bevy::{
    color::palettes::css::WHITE, 
    prelude::*
};

use crate::{components::*, GameWindow};

const BULLET_SPEED: f32 = 1200.;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_bullets_system)
            .add_systems(Update, despawn_bullet_system);
    }
}


pub fn spawn_bullet_system(
    player_transform: Single<&Transform, With<Player>>,
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mesh_handle = meshes.add(Mesh::from(Circle::new(2.)));
    let material_handle = materials.add(ColorMaterial::from_color(WHITE));

    c.spawn(Mesh2d(mesh_handle))
        .insert(MeshMaterial2d(material_handle))
        .insert(Transform {
            translation: player_transform.translation,
            rotation: player_transform.rotation,
            ..default()
        })
        .insert(Velocity::new(BULLET_SPEED, BULLET_SPEED))
        .insert(Bullet);
}

pub fn move_bullets_system(
    mut bullets: Query<(&mut Transform, &Velocity), With<Bullet>>,
    time: Res<Time>
) {
    bullets.iter_mut().for_each(|(mut transform, velocity)| {
        let forward_dir = transform.rotation * Vec3::Y;
        transform.translation += forward_dir * velocity.0.extend(0.) * time.delta_secs();
    });
}

pub fn despawn_bullet_system(
    mut c: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    window: Res<GameWindow>
) {
    let half_window_height = window.0.y / 2.;
    let half_window_width = window.0.x / 2.;

    let is_outside = |pos: Vec2| -> bool {
        pos.x > half_window_width ||
        pos.x < -(half_window_width) ||
        pos.y > half_window_height ||
        pos.y < -(half_window_height)
    };

    bullets.iter().for_each(|(bullet_entity, bullet_position)| {
        if is_outside(bullet_position.translation.truncate())  {
            c.entity(bullet_entity).despawn();
        }
    });
}
