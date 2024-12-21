use avian3d::prelude::*;
use bevy::{
    app::{App, Startup},
    input::common_conditions::input_just_pressed,
    prelude::*,
    DefaultPlugins,
};
use skyreaper::{systems, VIEWPORT_SIZE};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0., 0.71, 0.88)))
        .insert_resource(AmbientLight {
            brightness: 200.0,
            ..default()
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "SkyReaper".into(),
                    resolution: (500., 500. * (VIEWPORT_SIZE.y / VIEWPORT_SIZE.x)).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(), // XXX debug
        ))
        .add_systems(Startup, systems::setup)
        .add_systems(
            PreUpdate,
            systems::spawn_rocket.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(
            Update,
            (systems::spawn_airplane, systems::handle_world_collisions),
        )
        .add_systems(
            PostUpdate,
            (
                systems::handle_rocket_to_airplane_hit,
                systems::handle_airplane_to_airplane_hit,
            )
                .before(PhysicsSet::Sync),
        )
        .run();
}
