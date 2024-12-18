use std::f32::consts::PI;

use crate::animation::AnimationInfo;
use bevy::{
    animation::{animated_field, AnimationTarget},
    asset::Assets,
    math::Quat,
    prelude::*,
};

// See https://github.com/bevyengine/bevy/blob/5a94beb2391a12c13d022ad5bcb89e132061ec74/examples/animation/eased_motion.rs#L80
#[derive(Resource)]
pub struct AirplaneResource {
    animation_info: AnimationInfo,
    asset: Handle<Scene>,
}

impl AirplaneResource {
    pub fn create(
        asset_server: &Res<AssetServer>,
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
    ) -> AirplaneResource {
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
        }
    }

    pub fn configure(&self, entity_commands: &mut EntityCommands) {
        let mut animation_player = AnimationPlayer::default();
        animation_player
            .play(self.animation_info.node_index())
            .repeat();
        entity_commands.insert((
            SceneRoot(self.asset.clone()),
            self.animation_info.target_name().clone(),
            AnimationGraphHandle(self.animation_info.graph().clone()),
            animation_player,
            AnimationTarget {
                id: self.animation_info.target_id(),
                player: entity_commands.id(),
            },
        ));
    }
}
