use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    app::{App, Startup},
    asset::Assets,
    core_pipeline::core_3d::Camera3d,
    ecs::system::{Commands, ResMut},
    input::common_conditions::input_just_pressed,
    math::Vec3,
    pbr::StandardMaterial,
    prelude::*,
    render::camera::ScalingMode,
    transform::components::Transform,
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update.run_if(run_once))
        .add_systems(
            PostUpdate,
            kill_box
                .run_if(input_just_pressed(KeyCode::Space))
                .before(PhysicsSet::Sync),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    const VIEWPORT_HEIGHT: f32 = 6.0;

    // Ground
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(100.0, 0.5, 10.0),
        Mesh3d(meshes.add(Cuboid::new(100.0, 0.5, 10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -VIEWPORT_HEIGHT / 2.0, 0.0),
    ));

    // Airplane
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            ColliderDensity(0.0), // weightless
            LinearVelocity(Vec3::NEG_X),
            Transform::from_xyz(5.0, VIEWPORT_HEIGHT / 2.0 - 0.5, 0.0), //XXX position near top and offscreen right
        ))
        .with_child((
            SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/airplane.glb")),
            ),
            // Locally rotated
            Transform::from_rotation(Quat::from_rotation_y(-PI / 2.)),
        ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 0.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 0.0, VIEWPORT_HEIGHT).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

fn kill_box(mut commands: Commands, box_query: Query<(Entity, &RigidBody)>) {
    for (e, b) in box_query.iter() {
        if b.is_dynamic() {
            commands.entity(e).insert(ColliderDensity(5.0));
        }
    }
}

fn update(
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
