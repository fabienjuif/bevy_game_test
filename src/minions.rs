use crate::{
    common::*,
    health_bar::{Health, HealthBarBundle},
};
use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer},
    utils::default,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

const MINION_SCALE: f32 = 100.;
const DESTROY_MINIONS_AFTER_SECS: f32 = 120.;

pub struct MinionsPlugin;

#[derive(Component)]
struct Minion {
    destroy_timer: Timer,
}

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_move_minions, destroy_minions, hurt_player));
    }
}

pub fn spawn_minion(commands: &mut Commands, transform: &Transform, team: Team) {
    let mut rng = rand::thread_rng();

    let entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: team.color,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x + rng.gen_range(-20.0..20.0),
                    transform.translation.y + rng.gen_range(-20.0..20.0),
                    0.0,
                ),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(5.0, 5.),
            ActiveEvents::COLLISION_EVENTS,
            // Restitution::coefficient(2.),
            // Friction::coefficient(2.),
            Minion {
                // to avoid leaks
                // maybe a better option on top of that is to leach health every seconds on minions and make them die!
                destroy_timer: Timer::from_seconds(
                    DESTROY_MINIONS_AFTER_SECS,
                    bevy::time::TimerMode::Once,
                ),
            },
            Health::new(20.),
            team,
        ))
        .id();

    commands.spawn(HealthBarBundle::new(
        entity,
        Vec3::new(0.0, 15.0, 0.1),
        Vec2::new(10.0, 5.0),
    ));

    trace!("Spawning Minion: {:?}", entity);
}

fn update_move_minions(
    time: Res<Time>,
    // if we want minion to move over other minions later, maybe we want something like this instead of having 2 queries
    //      let mut combinations = query.iter_combinations_mut();
    //      while let Some([mut a, mut b]) = combinations.fetch_next() {
    mut query: Query<(&mut Transform, &Team), With<Minion>>,
    query_targets: Query<(&mut Transform, &Team), Without<Minion>>,
) {
    for (mut transform, team) in &mut query {
        // acquire new target
        let mut closer_target: &Transform = &Transform::from_xyz(0., 0., 0.);
        let mut found_target = false;
        for (target_transform, target_team) in &query_targets {
            if team.id == target_team.id {
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

fn destroy_minions(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Minion, &Transform, Entity)>,
) {
    for (mut minion, transform, entity) in &mut query {
        let escape_edges_of_the_world = transform.translation.x.abs() >= GAME_MAX_WIDTH / 2.
            || transform.translation.y.abs() >= GAME_MAX_HEIGHT / 2.;
        let too_old = minion.destroy_timer.tick(time.delta()).just_finished();

        if escape_edges_of_the_world || too_old {
            trace!("Unspawning Minion: {:?}", entity);
            commands.entity(entity).despawn_recursive();
            continue;
        }
    }
}

// maybe this is a bad idea to have a system per component since the collision event is having all contacts
// it makes us loop inside collision events multiple time
fn hurt_player(
    mut query_players: Query<(Entity, &Team, &mut Health), With<Player>>,
    query_minions: Query<(Entity, &Team), With<Minion>>,
    mut collision_events: EventReader<CollisionEvent>,
    // mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let minion = match query_minions
                    .get(*e1)
                    .ok()
                    .or_else(|| query_minions.get(*e2).ok())
                {
                    None => continue,
                    Some(p) => p,
                };

                // to avoid to borrow twice the query check with which entity the minion is resolved
                // and take the other one to check if this is a player, this hacky way of doing it
                // I am sure rust can resolve this better, here the first attempt in case someone
                // want to explain me how to resolve it with Rust
                //
                // let player = match query_players
                //     .get_mut(*e1)
                //     .ok()
                //     .or_else(|| query_players.get_mut(*e2).ok())
                // {
                //     None => continue,
                //     Some(p) => p,
                // };
                //
                let player = if query_players.contains(*e1) {
                    query_players.get_mut(*e1).ok()
                } else {
                    query_players.get_mut(*e2).ok()
                };
                let mut player = match player {
                    None => continue,
                    Some(p) => p,
                };

                // if they are from the same team, do nothing special
                if player.1.id == minion.1.id {
                    continue;
                }

                // hurt the player
                player.2.value -= 1.;
                if player.2.value < 0. {
                    player.2.value = 0.
                }

                trace!(
                    "minion {} collision with the player {}",
                    minion.1.id,
                    player.1.id
                )
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
