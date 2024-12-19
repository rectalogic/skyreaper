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
            (
                systems::spawn_airplane.run_if(input_just_pressed(KeyCode::Enter)),
                systems::spawn_rocket.run_if(input_just_pressed(KeyCode::Delete)),
            ),
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
