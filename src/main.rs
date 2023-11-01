mod common;
mod health_bar;
mod minions;
mod player;
mod racks;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_rapier2d::prelude::*;
use health_bar::HealthBarPlugin;
use minions::MinionsPlugin;
use player::LocalPlayerPlugin;
use racks::RacksPlugin;

#[derive(Component)]
struct Camera;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(LogPlugin {
            level: Level::TRACE,
            filter: "wgpu=error,bevy_render=warn,bevy_app=warn,bevy_ecs=warn,naga=warn,gilrs=warn"
                .to_string(),
        }),
        MinionsPlugin,
        RacksPlugin,
        HealthBarPlugin,
        LocalPlayerPlugin,
    ))
    // --- physics ---
    .add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        // RapierDebugRenderPlugin::default(),
    ))
    // removes gravity
    .insert_resource(RapierConfiguration {
        gravity: Vec2::new(0.0, 0.0),
        ..default()
    })
    // --- systems ---
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), Camera {}));
}
