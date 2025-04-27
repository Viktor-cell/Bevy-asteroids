use bevy::prelude::*;

use crate::GameWindow;

const MAX_NUMBER: usize = 10;
const SPAWN_COOLDOWN: f32 = 20.;

#[derive(Resource)]
pub struct AsteroidsInfo{
    pub count: usize,
    pub timer_to_next: Timer,
}

impl Default for AsteroidsInfo {
    fn default() -> Self {
        Self {
            count: 0,
            timer_to_next: Timer::from_seconds(20., TimerMode::Once)
        }
    }
}

pub fn spawn_asteroid(
    mut c: Commands,
    window: Res<GameWindow>,
) {
    todo!();
}

pub fn update_asteroid_info(
    mut c: Commands,
    window: Res<GameWindow>,
    mut asteroids_info: ResMut<AsteroidsInfo>,
    time: Res<Time>
) {
    asteroids_info.timer_to_next.tick(time.delta());
    todo!();
}
