use bevy::{animation::AnimationTargetId, asset::Assets, core::Name, prelude::*};

// See https://github.com/bevyengine/bevy/blob/5a94beb2391a12c13d022ad5bcb89e132061ec74/examples/animation/eased_motion.rs#L80
pub struct AnimationInfo {
    target_name: Name,
    target_id: AnimationTargetId,
    graph: Handle<AnimationGraph>,
    node_index: AnimationNodeIndex,
}

impl AnimationInfo {
    pub fn create<P, C>(
        animation_graphs: &mut Assets<AnimationGraph>,
        animation_clips: &mut Assets<AnimationClip>,
        property: P,
        curve: C,
    ) -> AnimationInfo
    where
        P: AnimatableProperty + Clone,
        C: AnimationCompatibleCurve<<P as AnimatableProperty>::Property> + Clone,
    {
        let mut animation_clip = AnimationClip::default();
        let animation_target_name = Name::new("animation");
        let animation_target_id = AnimationTargetId::from_name(&animation_target_name);

        animation_clip
            .add_curve_to_target(animation_target_id, AnimatableCurve::new(property, curve));
        let animation_clip_handle = animation_clips.add(animation_clip);
        let (animation_graph, animation_node_index) =
            AnimationGraph::from_clip(animation_clip_handle);

        let animation_graph_handle = animation_graphs.add(animation_graph);

        AnimationInfo {
            target_name: animation_target_name,
            target_id: animation_target_id,
            graph: animation_graph_handle,
            node_index: animation_node_index,
        }
    }

    pub fn target_name(&self) -> &Name {
        &self.target_name
    }
    pub fn target_id(&self) -> AnimationTargetId {
        self.target_id
    }
    pub fn graph(&self) -> &Handle<AnimationGraph> {
        &self.graph
    }
    pub fn node_index(&self) -> AnimationNodeIndex {
        self.node_index
    }
}
