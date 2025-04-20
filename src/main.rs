use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::mesh::shape::RegularPolygon,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (800.0, 600.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, init_player)
        .add_systems(Update, move_player_system)
        .run();
}

#[derive(Component)]
struct Player;

fn init_player(
    mut c: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    c.spawn(Camera2dBundle::default());

    let mesh_handle = meshes.add(Mesh::from(RegularPolygon::new(12.0, 3)));
    let material_handle = materials.add(Color::WHITE.into());

    c.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_handle),
            material: material_handle,
            ..default()
        },
        Player
    ));
}

const PLAYER_SPEED: f32 = 200.;
fn move_player_system(
    mut player_transform: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let mut player_transform = player_transform.single_mut();
    let window = window.get_single().unwrap();

    let forward_dir = player_transform.rotation * Vec3::Y;

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

    if keyboard.pressed(KeyCode::Space) {
        player_transform.translation += forward_dir * PLAYER_SPEED * time.delta_seconds();
    }
}
