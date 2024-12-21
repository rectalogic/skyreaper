use crate::models::airplane::{Airplane, AirplaneHit};
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
                Position::from_xyz(0., 0., PADDING + 10.),
                Quat::IDENTITY,
                Collider::half_space(Vec3::NEG_Z),
            ),
            // Front wall
            (
                Position::from_xyz(0., 0., -(PADDING + 6.)),
                Quat::IDENTITY,
                Collider::half_space(Vec3::Z),
            ),
        ]),
    ));

    // Light
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(0.0, VIEWPORT_SIZE.y, 1.0),
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
        Transform::from_xyz(0.0, 0.0, 6.).looking_at(Vec3::ZERO, Dir3::Y),
    ));
}

pub fn spawn_airplane(
    commands: Commands,
    mut airplane_resource: ResMut<AirplaneResource>,
    time: Res<Time>,
) {
    airplane_resource.tick(commands, time);
}

pub fn spawn_rocket(commands: Commands, rocket_resource: Res<RocketResource>) {
    rocket_resource.spawn(commands);
}

// It should be enough to check collisions.contains(),
// but avian has a bug and so we need to check for non-empty contacts too
// https://github.com/Jondolf/avian/issues/586
fn has_collision(collisions: &Res<Collisions>, worldbox: Entity, entity: Entity) -> bool {
    if let Some(contacts) = collisions.get(worldbox, entity) {
        return contacts.manifolds.iter().any(|m| !m.contacts.is_empty());
    }
    false
}

pub fn handle_world_collisions(
    mut commands: Commands,
    collisions: Res<Collisions>,
    worldbox: Query<Entity, With<WorldBox>>,
    rockets: Query<Entity, With<Rocket>>,
    airplanes: Query<(Entity, &ColliderParent), With<Airplane>>,
) {
    for wb in &worldbox {
        for rocket in &rockets {
            if has_collision(&collisions, wb, rocket) {
                commands.entity(rocket).despawn_recursive();
                println!("despawn rocket {rocket:?}"); //XXX
            }
        }
        for (airplane, airplane_parent) in &airplanes {
            if has_collision(&collisions, wb, airplane) {
                commands.entity(airplane_parent.get()).despawn_recursive();
                println!("despawn airplane {airplane:?}"); //XXX
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_rocket_to_airplane_hit(
    mut commands: Commands,
    collisions: Res<Collisions>,
    rockets: Query<Entity, With<Rocket>>,
    airplanes: Query<(Entity, &ColliderParent), (With<Airplane>, Without<AirplaneHit>)>,
) {
    for rocket in &rockets {
        for (airplane, collider_parent) in &airplanes {
            if collisions.contains(rocket, airplane) {
                // Mark plane hit, and remove uplift force so it falls
                commands.entity(airplane).insert(AirplaneHit);
                commands
                    .entity(collider_parent.get())
                    .insert(ExternalForce::ZERO);

                println!("airplane {airplane:?} hit by rocket"); //XXX
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_airplane_to_airplane_hit(
    mut commands: Commands,
    collisions: Res<Collisions>,
    hit_airplanes: Query<Entity, (With<Airplane>, With<AirplaneHit>)>,
    unhit_airplanes: Query<Entity, (With<Airplane>, Without<AirplaneHit>)>,
) {
    for hit_airplane in &hit_airplanes {
        for unhit_airplane in &unhit_airplanes {
            if collisions.contains(hit_airplane, unhit_airplane) {
                // Mark unhit plane as hit, and give it weight so it falls
                commands
                    .entity(unhit_airplane)
                    .insert((AirplaneHit, ColliderDensity(5.0)));
                println!("airplane {unhit_airplane:?} hit by hit airplane"); //XXX
            }
        }
    }
}
