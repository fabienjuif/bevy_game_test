mod castles;
mod common;
mod health;
mod minions;
mod physics;
mod player;
mod racks;
mod teams;

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_camera::CameraPlugin;
use bevy_rapier2d::prelude::*;
use bevy_turborand::prelude::*;
use castles::CastlesPlugin;
use health::HealthPlugin;
use minions::MinionsPlugin;
use physics::PhysicsPlugin;
use player::LocalPlayerPlugin;
use racks::RacksPlugin;
use teams::TeamsPlugin;

use fasthash::xx;

fn main() {
    let mut app = App::new();
    let seed = "12U8x";

    app.add_plugins((
        DefaultPlugins.set(LogPlugin {
            level: Level::TRACE,
            filter: "wgpu=error,bevy_render=warn,bevy_app=warn,bevy_ecs=warn,naga=warn,gilrs=warn,game::health=info,game::racks=info"
                .to_string(),
        }),
        RngPlugin::new().with_rng_seed(xx::hash64(seed)),
        PhysicsPlugin,
        TeamsPlugin,
        MinionsPlugin,
        RacksPlugin,
        CastlesPlugin,
        HealthPlugin,
        LocalPlayerPlugin,
    ))
    // --- camera ---
    .add_plugins((
        CameraPlugin,
        // CameraDebugPlugin,
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
    .run();
}
