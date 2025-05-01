use bevy::prelude::*;

use crate::{asteroid::AsteroidsInfo, components::{Asteroid, Player}};

struct ColisionsPlugin;

impl Plugin for ColisionsPlugin {
    fn build(&self, app: &mut App) {
        todo!();
    }
}

fn asteroid_player_colision_system(
    player_transform: Single<&Transform, With<Player>>,
    asteroids_transform: Query<&Transform, With<Asteroid>>
) {
    asteroids_transform.iter().for_each(|asteroid_transform| {
        todo!();
    });
}
