use bevy::{
    color::palettes::css::BLACK, 
    prelude::*
};

use crate::{
    components::{
        Asteroid,
        Health,
        LabelFollows
    },
    GameWindow
};

const FONT_SIZE: f32 = 20.;
const SHIFT_LEFT: f32 = 6.2;
const SHIFT_BOT: f32 = 10.;
pub fn spawn_labels_on_asteroids_system(
    mut c: Commands,
    asteroid: Query<(Entity, &Transform, &Health), With<Asteroid>>,
    already_spawned: Query<&LabelFollows>,
    screen: Res<GameWindow>
) {
    for (asteroid_entity, asteroid_transform, asteroid_health) in asteroid {
        if already_spawned.iter().any(|spawned| spawned.0 == asteroid_entity) {
            continue;
        }

        c.spawn(Node {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            left: Val::Px(asteroid_transform.translation.x + screen.0.x / 2. - SHIFT_LEFT),
            bottom: Val::Px(asteroid_transform.translation.y + screen.0.y / 2. - SHIFT_BOT),
            ..default()
        })
            .insert(Text::new(format!("{}", asteroid_health.0)))
            .insert(TextColor(BLACK.into()))
            .insert(TextFont {
                font_size: FONT_SIZE,
                font_smoothing: bevy::text::FontSmoothing::AntiAliased,
                ..default()
            })
            .insert(LabelFollows(asteroid_entity));
    }
}

pub fn update_labels_on_asteroid_system(
    mut c: Commands,
    asteroids: Query<(&Transform, &Health), With<Asteroid>>,
    screen: Res<GameWindow>,
    text: Query<(&mut Node, &LabelFollows, Entity, &mut Text)>
) {
    for (mut text_node, label_follows, label_entity,mut label_text) in text {
        if let Ok((asteroid_transform, asteroid_health)) = asteroids.get(label_follows.0) {
            text_node.left = Val::Px(asteroid_transform.translation.x + screen.0.x / 2. - SHIFT_LEFT);
            text_node.bottom = Val::Px(asteroid_transform.translation.y + screen.0.y / 2. - SHIFT_BOT);
            label_text.0 = format!("{}", asteroid_health.0)
        } else {
            c.entity(label_entity).despawn();
        }
    }
}
