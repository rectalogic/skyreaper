use std::{
    f32::consts::{FRAC_PI_2, PI},
    time::Duration,
};

use crate::animation::AnimationInfo;
use avian3d::prelude::*;
use bevy::{
    animation::{animated_field, AnimationTarget},
    asset::Assets,
    ecs::system::Commands,
    math::Quat,
    math::Vec3,
    prelude::*,
    transform::components::Transform,
};

use crate::VIEWPORT_SIZE;

#[derive(Component)]
pub struct Airplane;

#[derive(Component)]
pub struct AirplaneHit;

#[derive(Resource)]
pub struct AirplaneResource {
    animation_info: AnimationInfo,
    asset: Handle<Scene>,
    spawn_timer: Timer,
}

impl AirplaneResource {
    pub fn new(
        asset_server: &Res<AssetServer>,
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
    ) -> Self {
        let rotation_curve = EasingCurve::new(
            Quat::from_rotation_z(-PI / 16.0),
            Quat::from_rotation_z(PI / 16.0),
            EaseFunction::BackInOut,
        )
        // 1.0 seconds for each cycle
        .reparametrize_linear(interval(0.0, 1.0).unwrap())
        .expect("this curve has bounded domain, so this should never fail")
        .ping_pong()
        .expect("this curve has bounded domain, so this should never fail");
        let animation_info = AnimationInfo::create(
            animation_graphs,
            animation_clips,
            animated_field!(Transform::rotation),
            rotation_curve,
        );

        AirplaneResource {
            animation_info,
            asset: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/airplane.glb")),
            spawn_timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
        }
    }

    pub fn tick(&mut self, mut commands: Commands, time: Res<Time>) {
        self.spawn_timer.tick(time.delta());
        if !self.spawn_timer.finished() {
            return;
        }
        commands
            .spawn((
                RigidBody::Dynamic,
                LinearVelocity(Vec3::NEG_X),
                ExternalForce::new(1.177 * Vec3::Y),
                Transform::from_xyz(VIEWPORT_SIZE.x / 2., VIEWPORT_SIZE.y / 2.0 - 0.5, 0.0), //XXX position near top and offscreen right
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
                        let mut airplane_commands = parent.spawn((
                            Airplane,
                            SceneRoot(self.asset.clone()),
                            ColliderDensity(1.0),
                            // T shaped collider to fit plane
                            // Use ColliderParent to find root RigidBody entity
                            Collider::compound(vec![
                                // Body
                                (
                                    Vec3::ZERO,
                                    Quat::IDENTITY,
                                    Collider::cuboid(0.25, 0.25, 0.8),
                                ),
                                // Wings
                                (
                                    Vec3 {
                                        z: 0.15,
                                        ..default()
                                    },
                                    Quat::IDENTITY,
                                    Collider::cuboid(0.8, 0.35, 0.25),
                                ),
                            ]),
                        ));
                        airplane_commands
                            .insert(self.create_animation_bundle(airplane_commands.id()));
                    });
            });
    }

    fn create_animation_bundle(&self, entity: Entity) -> impl Bundle {
        let mut animation_player = AnimationPlayer::default();
        animation_player
            .play(self.animation_info.node_index())
            .repeat();
        (
            self.animation_info.target_name().clone(),
            AnimationGraphHandle(self.animation_info.graph().clone()),
            animation_player,
            AnimationTarget {
                id: self.animation_info.target_id(),
                player: entity,
            },
        )
    }
}
