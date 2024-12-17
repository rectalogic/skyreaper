use std::f32::consts::PI;

use bevy::{
    animation::{animated_field, AnimationTarget, AnimationTargetId},
    asset::Assets,
    core::Name,
    math::Quat,
    prelude::*,
};

// See https://github.com/bevyengine/bevy/blob/5a94beb2391a12c13d022ad5bcb89e132061ec74/examples/animation/eased_motion.rs#L80
#[derive(Resource)]
pub struct AirplaneInfo {
    animation_info: AnimationInfo,
    asset: Handle<Scene>,
}
struct AnimationInfo {
    target_name: Name,
    target_id: AnimationTargetId,
    graph: Handle<AnimationGraph>,
    node_index: AnimationNodeIndex,
}

impl AirplaneInfo {
    pub fn create(
        asset_server: &Res<AssetServer>,
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
    ) -> AirplaneInfo {
        let mut animation_clip = AnimationClip::default();
        let animation_target_name = Name::new("airplane");
        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);

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

        animation_clip.add_curve_to_target(
            animation_target_id,
            AnimatableCurve::new(animated_field!(Transform::rotation), rotation_curve),
        );
        let animation_clip_handle = animation_clips.add(animation_clip);
        let (animation_graph, animation_node_index) =
            AnimationGraph::from_clip(animation_clip_handle);

        let animation_graph_handle = animation_graphs.add(animation_graph);

        AirplaneInfo {
            animation_info: AnimationInfo {
                target_name: animation_target_name,
                target_id: animation_target_id,
                graph: animation_graph_handle,
                node_index: animation_node_index,
            },
            asset: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/airplane.glb")),
        }
    }

    pub fn configure_airplane(&self, entity_commands: &mut EntityCommands) {
        let mut animation_player = AnimationPlayer::default();
        animation_player
            .play(self.animation_info.node_index)
            .repeat();
        entity_commands.insert((
            SceneRoot(self.asset.clone()),
            self.animation_info.target_name.clone(),
            AnimationGraphHandle(self.animation_info.graph.clone()),
            animation_player,
            AnimationTarget {
                id: self.animation_info.target_id,
                player: entity_commands.id(),
            },
        ));
    }
}
