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
use skyreaper::{models::airplane::AirplaneResource, systems, VIEWPORT_HEIGHT};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(), // XXX debug
        ))
        .add_systems(Startup, setup)
        .add_systems(
            PreUpdate,
            systems::spawn_airplane.run_if(input_just_pressed(KeyCode::Enter)),
        )
        .add_systems(Update, (systems::update.run_if(run_once), log_collisions))
        .add_systems(
            PostUpdate,
            systems::kill_box
                .run_if(input_just_pressed(KeyCode::Space))
                .before(PhysicsSet::Sync),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_clips: ResMut<Assets<AnimationClip>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(AirplaneResource::create(
        &asset_server,
        &mut animation_graphs,
        &mut animation_clips,
    ));

    // Ground
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(100.0, 0.5, 10.0),
        Mesh3d(meshes.add(Cuboid::new(100.0, 0.5, 10.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -VIEWPORT_HEIGHT / 2.0, 0.0),
        CollidingEntities::default(), // XXX query collisions with this
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

fn log_collisions(query: Query<(Entity, &CollidingEntities)>) {
    for (entity, colliding_entities) in &query {
        if !colliding_entities.0.is_empty() {
            println!(
                "{:?} is colliding with the following entities: {:?}",
                entity, colliding_entities
            );
        }
    }
}
