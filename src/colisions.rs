use bevy::prelude::*;

use crate::{
    asteroid::AsteroidsInfo, 
    components::{
        Asteroid,
        Bullet,
        Collider,
        Health,
        Player
    }
};

pub struct ColisionsPlugin;

impl Plugin for ColisionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, bullet_asteroid_collision_system)
            .add_systems(Update, asteroid_player_collision_system);
    }
}

fn asteroid_player_collision_system(
    player_query: Query<(&Transform, &Collider), With<Player>>,
    asteroid_query: Query<(&Transform, &Collider), With<Asteroid>>,
) {
    let (player_transform, player_collider) = player_query.single().unwrap();

    for (asteroid_transform, asteroid_collider) in asteroid_query.iter() {
        if circle_circle_collision((asteroid_transform, asteroid_collider), (player_transform, player_collider)) {
            info!("Player died");
        }
    }
}

fn bullet_asteroid_collision_system(
    mut commands: Commands,
    mut asteroid_query: Query<(&Transform, &Collider, Entity, &mut Health), With<Asteroid>>,
    bullet_query: Query<(&Transform, &Collider, Entity), With<Bullet>>,
    mut asteroid_info: ResMut<AsteroidsInfo>,
) {
    for (asteroid_transform, asteroid_collider, asteroid_entity, mut asteroid_health) in asteroid_query.iter_mut() {
        for (bullet_transform, bullet_collider, bullet_entity) in bullet_query.iter() {
            if circle_circle_collision((asteroid_transform, asteroid_collider), (bullet_transform, bullet_collider)) {
                commands.entity(bullet_entity).despawn();
                asteroid_health.0 -= 1;

                if asteroid_health.0 == 0 {
                    commands.entity(asteroid_entity).despawn();
                    asteroid_info.count -= 1;
                }

                break;
            }
        }
    }
}

fn circle_circle_collision(
    (a_transform, a_collider): (&Transform, &Collider),
    (b_transform, b_collider): (&Transform, &Collider),
) -> bool {
    let distance = a_transform.translation.distance(b_transform.translation);
    let radius_sum = a_collider.radius + b_collider.radius;
    distance < radius_sum
}
