mod audio;
mod castles;
mod common;
mod health;
mod minions;
mod physics;
mod player;
mod racks;
mod teams;

use audio::AudioPlugin;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    DefaultPlugins,
};
use bevy_cameraman::CameraPlugin;
use bevy_rapier2d::prelude::*;
use bevy_turborand::prelude::*;
use castles::CastlesPlugin;
use health::HealthPlugin;
use minions::MinionsPlugin;
use physics::PhysicsPlugin;
use player::LocalPlayerPlugin;
use racks::RacksPlugin;
use teams::TeamsPlugin;
use xxhash_rust::xxh3::xxh3_64;

const AUDIO_SCALE: f32 = 1. / 100.0;

fn main() {
    let mut app = App::new();
    let seed = b"13U2x";

    app.add_plugins((
        DefaultPlugins
            .set(LogPlugin {
                level: Level::TRACE,
                filter: [
                    "wgpu=error",
                    "bevy_render=warn,bevy_app=warn,bevy_ecs=warn",
                    "naga=warn",
                    "gilrs=warn",
                    "game::health=info,game::racks=info",
                ]
                .join(","),
            })
            .set(bevy::audio::AudioPlugin {
                spatial_scale: bevy::audio::SpatialScale::new_2d(AUDIO_SCALE),
                ..default()
            }),
        RngPlugin::new().with_rng_seed(xxh3_64(seed)),
        PhysicsPlugin,
        TeamsPlugin,
        MinionsPlugin,
        RacksPlugin,
        CastlesPlugin,
        HealthPlugin,
        LocalPlayerPlugin,
        AudioPlugin,
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
