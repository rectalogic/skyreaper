use std::f32::consts::FRAC_PI_2;

use crate::{airplane::AirplaneInfo, VIEWPORT_HEIGHT};
use avian3d::prelude::*;
use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::StandardMaterial,
    prelude::*,
    transform::components::Transform,
};

pub fn spawn_airplane(mut commands: Commands, airplane_info: Res<AirplaneInfo>) {
    commands
        .spawn((
            RigidBody::Dynamic,
            // T shaped collider to fit plane
            Collider::compound(vec![
                (
                    Vec3::ZERO,
                    Quat::IDENTITY,
                    Collider::cuboid(0.8, 0.25, 0.25),
                ),
                (
                    Vec3 {
                        x: -0.15,
                        ..default()
                    },
                    Quat::IDENTITY,
                    Collider::cuboid(0.25, 0.25, 0.8),
                ),
            ]),
            ColliderDensity(0.0), // weightless
            LinearVelocity(Vec3::NEG_X),
            Transform::from_xyz(5.0, VIEWPORT_HEIGHT / 2.0 - 0.5, 0.0), //XXX position near top and offscreen right
            Visibility::Inherited,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    // Locally rotated
                    Transform::from_rotation(Quat::from_rotation_y(-FRAC_PI_2)),
                    Visibility::Inherited,
                ))
                .with_children(|parent| {
                    let mut airplane_commands = parent.spawn((Transform::default(),));
                    airplane_info.configure_airplane(&mut airplane_commands);
                });
        });
}

pub fn kill_box(mut commands: Commands, box_query: Query<(Entity, &RigidBody)>) {
    for (e, b) in box_query.iter() {
        if b.is_dynamic() {
            commands.entity(e).insert(ColliderDensity(5.0));
        }
    }
}

pub fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = *camera_query;
    /*
    if let Some(world) = camera.ndc_to_world(
        camera_transform,
        Vec3 {
            x: 0.,
            y: 1.,
            z: 0.01,
        },
    ) {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::WHITE)),
            Transform::from_translation(world),
        ));
    }
    */

    if let Some(viewport) = camera.logical_viewport_rect() {
        if let Ok(ray) = camera.viewport_to_world(
            camera_transform,
            Vec2 {
                x: viewport.max.x / 2.0,
                y: viewport.min.y,
            },
        ) {
            // Project along frustum to origin (i.e. distance of camera in Z)
            let world = ray.get_point(camera_transform.translation().z);
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(materials.add(Color::WHITE)),
                Transform::from_translation(world),
            ));
        }
    }
}
