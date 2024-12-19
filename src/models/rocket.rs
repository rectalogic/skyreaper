use std::f32::consts::PI;

use avian3d::prelude::*;
use bevy::{
    animation::{animated_field, AnimationTarget, AnimationTargetId},
    asset::Assets,
    core::Name,
    math::Quat,
    prelude::*,
};

use crate::VIEWPORT_SIZE;

#[derive(Component)]
pub struct Rocket;

#[derive(Resource)]
pub struct RocketResource {
    asset: Handle<Scene>,
}

impl RocketResource {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        RocketResource {
            asset: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/rocket.glb")),
        }
    }

    pub fn spawn(&self, mut commands: Commands) {
        commands.spawn((
            Rocket,
            Name::new("Rocket"),
            RigidBody::Dynamic,
            Collider::cylinder(0.1, 1.0),
            ColliderDensity(0.01), // weightless
            LinearVelocity(Vec3::Y * 25.0),
            Transform::from_xyz(0.0, -VIEWPORT_SIZE.y / 2.0 + 1.0, 0.0)
                .with_scale(Vec3::splat(0.5)), //XXX position
            SceneRoot(self.asset.clone()),
        ));
    }
}
