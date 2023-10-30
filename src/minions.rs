use crate::common::*;
use bevy::{
    prelude::{
        trace, App, Color, Commands, Component, DespawnRecursiveExt, Entity, Plugin, Query, Res,
        ResMut, Resource, Transform, Update, Vec2, With, Without,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    utils::default,
};
use rand::Rng;

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
                transform: Transform::from_xyz(
                    rng.gen_range(-20.0..20.0),
                    rng.gen_range(-20.0..20.0),
                    0.0,
                ),
                ..default()
            },
            Minion {},
            Team("b".to_string()),
        ))
        .id();

    trace!("Spawning Minion: {:?}", id);
}

fn update_move_minions(
    mut commands: Commands,
    time: Res<Time>,
    // if we want minion to move over other minions later, maybe we want something like this instead of having 2 queries
    //      let mut combinations = query.iter_combinations_mut();
    //      while let Some([mut a, mut b]) = combinations.fetch_next() {
    mut query: Query<(&mut Transform, Entity, &Team), With<Minion>>,
    query_targets: Query<(&mut Transform, &Team), Without<Minion>>,
) {
    for (mut transform, entity, team) in &mut query {
        // unspawn minions time to time
        if transform.translation.x.abs() >= GAME_MAX_WIDTH / 2.
            || transform.translation.y.abs() >= GAME_MAX_HEIGHT / 2.
        {
            trace!("Unspawning Minion: {:?}", entity);
            commands.entity(entity).despawn_recursive();
            continue;
        }

        // acquire new target
        let mut closer_target: &Transform = &Transform::from_xyz(0., 0., 0.);
        let mut found_target = false;
        for (target_transform, target_team) in &query_targets {
            if team.0 == target_team.0 {
                continue;
            }

            if !found_target {
                found_target = true;
                closer_target = target_transform;
                continue;
            }
            if transform
                .translation
                .distance_squared(closer_target.translation)
                > transform
                    .translation
                    .distance_squared(target_transform.translation)
            {
                closer_target = target_transform;
            }
        }

        // move toward the target
        if !found_target {
            continue;
        }
        let normalized_target_position =
            (closer_target.translation - transform.translation).normalize();
        transform.translation.x +=
            time.delta_seconds() * MINION_SCALE * normalized_target_position.x;
        transform.translation.y +=
            time.delta_seconds() * MINION_SCALE * normalized_target_position.y;
    }
}

fn spawn_minions(mut commands: Commands, time: Res<Time>, mut spawner: ResMut<MinionsSpawner>) {
    if spawner.timer.tick(time.delta()).just_finished() {
        for _ in 0..spawner.count {
            spawn_minion(&mut commands);
        }
    }
}
