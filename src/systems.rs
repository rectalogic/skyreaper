use crate::models::{airplane::AirplaneResource, rocket::RocketResource};
use crate::VIEWPORT_SIZE;
use avian3d::prelude::*;
use bevy::{
    asset::Assets,
    core_pipeline::core_3d::Camera3d,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    pbr::StandardMaterial,
    prelude::*,
    render::camera::ScalingMode,
    transform::components::Transform,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(AirplaneResource::new(
        &asset_server,
        &mut animation_graphs,
        &mut animation_clips,
    ));
    commands.insert_resource(RocketResource::new(&asset_server));

    // Ground
    const FLOOR_HEIGHT: f32 = 0.5;
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(100.0, FLOOR_HEIGHT, 10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -VIEWPORT_SIZE.y / 2.0, 0.0),
    ));
    // World - colliders surrounding the world so nothing can escape
    commands
        .spawn((RigidBody::Static, Transform::default()))
        .with_children(|parent| {
            const PADDING: f32 = 2.;
            // Ceiling
            parent.spawn((
                Collider::half_space(Vec3::NEG_Y),
                Transform::from_xyz(0., PADDING + VIEWPORT_SIZE.y / 2., 0.),
            ));
            // Floor
            parent.spawn((
                Collider::half_space(Vec3::Y),
                Transform::from_xyz(0., (-VIEWPORT_SIZE.y + FLOOR_HEIGHT) / 2., 0.),
            ));
            // Right wall
            parent.spawn((
                Collider::half_space(Vec3::NEG_X),
                Transform::from_xyz(PADDING + VIEWPORT_SIZE.x / 2., 0., 0.),
            ));
            // Left wall
            parent.spawn((
                Collider::half_space(Vec3::X),
                Transform::from_xyz(-(PADDING + VIEWPORT_SIZE.x / 2.), 0., 0.),
            ));
            // Back wall
            parent.spawn((
                Collider::half_space(Vec3::NEG_Z),
                Transform::from_xyz(0., 0., PADDING + 2.),
            ));
            // Front wall
            parent.spawn((
                Collider::half_space(Vec3::Z),
                Transform::from_xyz(0., 0., -(PADDING + 2.)),
            ));
        });

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 4.0, 0.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: VIEWPORT_SIZE.x,
                height: VIEWPORT_SIZE.y,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 0.0, 1.).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

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
