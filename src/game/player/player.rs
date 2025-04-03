use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{plugin::RapierContext, prelude::*};

use super::{camera_controller, input::*, player_movement::*};
use crate::game::{
    level::targets::{DeadTarget, Target},
    shooting,
};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(shooting::tracer::TracerPlugin)
            .init_resource::<PlayerInput>()
            .add_systems(
                Update,
                (
                    update_movement_input,
                    update_player,
                    camera_controller::update_camera_controller,
                ),
            )
            .add_systems(FixedUpdate, update_movement)
            .add_systems(Startup, init_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec3,
    pub gravity: f32,
    pub speed: f32,
}
fn init_player(mut commands: Commands) {
    let fov = 103.0_f32.to_radians();
    let camera_entity = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::IDENTITY,
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: fov,
                    ..default()
                }),
                ..default()
            },
            camera_controller::CameraController {
                sensitivity: 0.035,
                rotation: Vec2::ZERO,
                rotation_lock: 88.0,
            },
        ))
        .id();

    let player_entity = commands
        .spawn((
            Player {
                velocity: Vec3::ZERO,
                gravity: 9.8,
                speed: 20.0,
            },
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., 10., 0.)),
                ..Default::default()
            },
            Collider::cuboid(1., 10., 1.),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
        ))
        .id();

    commands.entity(player_entity).add_child(camera_entity);
}

fn update_player(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
        &mut GlobalTransform,
        &mut Camera,
    )>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    target_query: Query<Entity, With<Target>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok((_player, transform, global_transform, camera)) = player_query.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let Some(ray) = camera.viewport_to_world(
                &global_transform,
                Vec2::new(window.width() / 2., window.height() / 2.),
            ) else {
                return;
            };
            let hit = rapier_context.cast_ray_and_get_normal(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                true,
                QueryFilter::default(),
            );
            if let Some((entity, ray_intersection)) = hit {
                if let Ok(_entity) = target_query.get(entity) {
                    commands.entity(entity).insert(DeadTarget);
                }
                //spawn tracer and check collisions
                let tracer_material = StandardMaterial {
                    base_color: Color::srgb(1., 1., 0.),
                    unlit: true,
                    ..default()
                };

                commands.spawn((
                    PbrBundle {
                        transform: Transform::from_translation(Vec3::splat(f32::MAX)),
                        mesh: meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 1.0))),
                        material: materials.add(tracer_material),
                        ..default()
                    },
                    shooting::tracer::BulletTracer::new(
                        transform.translation,
                        ray_intersection.point,
                        100.,
                    ),
                ));
            }
        }
    }
}
