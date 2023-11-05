mod castles;
mod common;
mod health;
mod minions;
mod player;
mod racks;
mod teams;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_rapier2d::prelude::*;
use castles::CastlesPlugin;
use health::HealthPlugin;
use minions::MinionsPlugin;
use player::LocalPlayerPlugin;
use racks::RacksPlugin;
use teams::TeamsPlugin;

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
        TeamsPlugin,
        MinionsPlugin,
        RacksPlugin,
        CastlesPlugin,
        HealthPlugin,
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
