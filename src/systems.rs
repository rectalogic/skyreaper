use crate::models::{airplane::AirplaneResource, rocket::RocketResource};
use avian3d::prelude::*;
use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::StandardMaterial,
    prelude::*,
    transform::components::Transform,
};

pub fn spawn_airplane(commands: Commands, airplane_resource: Res<AirplaneResource>) {
    airplane_resource.spawn(commands);
}

pub fn spawn_rocket(commands: Commands, rocket_resource: Res<RocketResource>) {
    rocket_resource.spawn(commands);
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
