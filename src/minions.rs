use bevy::{
    prelude::{
        trace, App, Color, Commands, Component, DespawnRecursiveExt, Entity, Plugin, Query, Res,
        ResMut, Resource, Transform, Update, Vec2, Vec3, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    utils::default,
};
use rand::Rng;

use crate::common::*;

const MINION_SCALE: f32 = 100.;

pub struct MinionsPlugin;

#[derive(Resource)]
struct MinionsSpawner {
    timer: Timer,
    count: u32,
}

#[derive(Component)]
struct Minion;

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MinionsSpawner {
            timer: Timer::from_seconds(3., TimerMode::Repeating),
            count: 50,
        })
        .add_systems(Update, (update_move_minions, spawn_minions));
    }
}

// TODO: Use global tranform?
// TODO: use a seed random maybe Bevy has one
// TODO: Target should be updated in the main loop (and not being randomize)
pub fn spawn_minion(commands: &mut Commands) {
    let mut rng = rand::thread_rng();

    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1., 0., 0.),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Enemy {},
            Minion {},
            Target {
                position: Vec3::new(
                    rng.gen_range(-1000.0..1000.0),
                    rng.gen_range(-1000.0..1000.0),
                    0.,
                ),
            },
        ))
        .id();

    trace!("Spawning Minion: {:?}", id);
}

fn update_move_minions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Target, Entity), With<Minion>>,
) {
    for (mut transform, target, entity) in &mut query {
        let normalized_target_position = target.position.normalize();
        transform.translation.x +=
            time.delta_seconds() * MINION_SCALE * normalized_target_position.x;
        transform.translation.y +=
            time.delta_seconds() * MINION_SCALE * normalized_target_position.y;

        if transform.translation.x.abs() >= GAME_MAX_WIDTH / 2.
            || transform.translation.y.abs() >= GAME_MAX_HEIGHT / 2.
        {
            trace!("Unspawning Minion: {:?}", entity);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_minions(mut commands: Commands, time: Res<Time>, mut spawner: ResMut<MinionsSpawner>) {
    if spawner.timer.tick(time.delta()).just_finished() {
        for _ in 0..spawner.count {
            spawn_minion(&mut commands);
        }
    }
}
