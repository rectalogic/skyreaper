use crate::models::airplane::Airplane;
use crate::models::rocket::Rocket;
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

#[derive(Component)]
pub struct WorldBox;

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
        Mesh3d(meshes.add(Cuboid::new(100.0, FLOOR_HEIGHT, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -VIEWPORT_SIZE.y / 2.0, 0.0),
    ));

    // World - colliders surrounding the world so nothing can escape
    const PADDING: f32 = 2.;
    commands.spawn((
        WorldBox,
        RigidBody::Static,
        Transform::default(),
        Collider::compound(vec![
            // Ceiling
            (
                Position::from_xyz(0., PADDING + VIEWPORT_SIZE.y / 2., 0.),
                Quat::IDENTITY,
                Collider::half_space(Vec3::NEG_Y),
            ),
            // Floor
            (
                Position::from_xyz(0., (-VIEWPORT_SIZE.y + FLOOR_HEIGHT) / 2., 0.),
                Quat::IDENTITY,
                Collider::half_space(Vec3::Y),
            ),
            // Right wall
            (
                Position::from_xyz(PADDING + VIEWPORT_SIZE.x / 2., 0., 0.),
                Quat::IDENTITY,
                Collider::half_space(Vec3::NEG_X),
            ),
            // Left wall
            (
                Position::from_xyz(-(PADDING + VIEWPORT_SIZE.x / 2.), 0., 0.),
                Quat::IDENTITY,
                Collider::half_space(Vec3::X),
            ),
            // Back wall
            (
                Position::from_xyz(0., 0., PADDING + 2.),
                Quat::IDENTITY,
                Collider::half_space(Vec3::NEG_Z),
            ),
            // Front wall
            (
                Position::from_xyz(0., 0., -(PADDING + 2.)),
                Quat::IDENTITY,
                Collider::half_space(Vec3::Z),
            ),
        ]),
    ));

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

pub fn kill_box(
    mut commands: Commands,
    box_query: Query<(Entity, &ColliderParent), With<Airplane>>,
) {
    for (e, p) in box_query.iter() {
        commands.entity(e).insert(ColliderDensity(5.0));
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

pub fn log_collisions(
    mut commands: Commands,
    collisions: Res<Collisions>,
    worldbox: Query<Entity, With<WorldBox>>,
    rockets: Query<Entity, With<Rocket>>,
) {
    // let i = collisions.get_internal();
    // if !i.is_empty() {
    //     dbg!(i);
    // }
    for wb in &worldbox {
        //dbg!(&colliding_entities.0); // XXX this always has everything, even once we despawn? https://github.com/Jondolf/avian/issues/533
        for rocket in &rockets {
            if collisions.contains(wb, rocket) {
                dbg!(&collisions);
                println!("despawn {rocket:?}");
                commands.entity(rocket).despawn();
            }
        }
        // if !colliding_entities.0.is_empty() {
        //     println!(
        //         "{:?} is colliding with the following entities: {:?}",
        //         entity, colliding_entities
        //     );
        // }
        // for e in colliding_entities.0.iter() {
        //     println!("{:?} is colliding with {:?}", entity, e);
        // }
    }
}
