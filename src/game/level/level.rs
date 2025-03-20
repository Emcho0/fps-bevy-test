use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
    }
}

fn init_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let level_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.spawn_bundle((
        Collider::cuboid(1000., 0., 1000.),
        PbrBundle {
            material: level_material.clone(),
            transform: Transform::IDENTITY,
            mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10000.))),
            ..default()
        },
    ));

    commands.spawn_bundle((
        Collider::cuboid(30., 30., 30.),
        PbrBundle {
            material: level_material.clone(),
            transform: Transform::from_xyz(0., 0., -100.),
            mesh: meshes.add(Cuboid::from_length(60.)),
            ..default()
        },
    ));

    commands.spawn_bundle((
        Collider::cuboid(30., 30., 30.),
        PbrBundle {
            material: level_material.clone(),
            transform: Transform::from_xyz(100., 200., 100.).looking_at(Vec3::ZERO, Vec3::Y),
            mesh: meshes.add(Cuboid::from_length(60.)),
            ..default()
        },
    ));
}
