use bevy::{
    prelude::*,
    render::mesh::shape::Circle,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
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
    player_transform: &Transform,
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mesh_handle = meshes.add(Mesh::from(Circle::new(2.)));
    let material_handle = materials.add(Color::WHITE.into());

    c.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(mesh_handle),
        material: material_handle,
        transform: *player_transform,
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
        transform.translation += forward_dir * velocity.0.extend(0.) * time.delta_seconds();
    });
}

pub fn despawn_bullet_system(
    mut c: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    window: Res<GameWindow>
) {

    let is_outside = |pos: Vec2| -> bool {
        pos.x > window.0.x / 2. ||
        pos.x < -(window.0.x / 2.) ||
        pos.y > window.0.y / 2. ||
        pos.y < -(window.0.y / 2.)
    };

    bullets.iter().for_each(|(bullet_entity, bullet_position)| {
        if is_outside(bullet_position.translation.truncate())  {
            c.entity(bullet_entity).despawn();
        }
    });
}
