use std::f32::consts::PI;

use bevy::{
    animation::{animated_field, AnimationTarget, AnimationTargetId},
    asset::Assets,
    core::Name,
    math::Quat,
    prelude::*,
};

#[derive(Resource)]
pub struct RocketResource {
    asset: Handle<Scene>,
}

impl RocketResource {
    pub fn create(asset_server: &Res<AssetServer>) -> RocketResource {
        RocketResource {
            asset: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/rocket.glb")),
        }
    }

    pub fn configure(&self, entity_commands: &mut EntityCommands) {
        // let mut animation_player = AnimationPlayer::default();
        // animation_player
        //     .play(self.animation_info.node_index)
        //     .repeat();
        // entity_commands.insert((
        //     SceneRoot(self.asset.clone()),
        //     self.animation_info.target_name.clone(),
        //     AnimationGraphHandle(self.animation_info.graph.clone()),
        //     animation_player,
        //     AnimationTarget {
        //         id: self.animation_info.target_id,
        //         player: entity_commands.id(),
        //     },
        // ));
    }
}
