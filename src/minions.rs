use crate::{
    common::*,
    health_bar::{Health, HealthBarBundle},
    teams::Team,
};
use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer},
    utils::{default, HashMap},
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

const MINION_SCALE: f32 = 100.;
const DESTROY_MINIONS_AFTER_SECS: f32 = 120.;
const DECAY_VALUE_PER_SEC: f32 = 10.;
const REWARDS_GOLD: f32 = 1.;

pub struct MinionsPlugin;

#[derive(Component)]
struct Minion {
    destroy_timer: Timer,
}

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_move_minions,
                destroy_minions,
                check_collisions_players,
                check_collisions_minions,
                decay_life,
            ),
        );
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
            Rewards { gold: REWARDS_GOLD },
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
    mut query: Query<(&mut Transform, Entity, &Team, Option<&Minion>)>,
) {
    let mut closest_translations: HashMap<Entity, Vec3> = HashMap::new();

    let mut combinations = query.iter_combinations_mut();
    while let Some(
        [(transform_a, entity_a, team_a, minion_a), (transform_b, entity_b, team_b, minion_b)],
    ) = combinations.fetch_next()
    {
        if minion_a.is_none() && minion_b.is_none() {
            continue;
        }

        if team_a.id == team_b.id {
            continue;
        }

        let (minion_transform, minion_entity, target_translation) = if minion_a.is_some() {
            (transform_a, entity_a, transform_b.translation)
        } else {
            (transform_b, entity_b, transform_a.translation)
        };

        // found a translation for this entity
        // but this is farther combination so we do nothing
        if let Some(closest_translation) = closest_translations.get(&minion_entity) {
            if minion_transform
                .translation
                .distance_squared(*closest_translation)
                <= minion_transform
                    .translation
                    .distance_squared(target_translation)
            {
                continue;
            }
        }

        // we are here if we found a closer combination
        // so we insert it and move toward this combination
        closest_translations.insert(minion_entity, target_translation);
    }

    for (entity, translation) in closest_translations {
        if let Ok((mut transform, _, _, _)) = query.get_mut(entity) {
            let normalized_target_position = (translation - transform.translation).normalize();

            transform.translation.x +=
                time.delta_seconds() * MINION_SCALE * normalized_target_position.x;
            transform.translation.y +=
                time.delta_seconds() * MINION_SCALE * normalized_target_position.y;
        }
    }
}

fn destroy_minions(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Minion, &Transform, &Health, Entity)>,
) {
    let mut kill = |entity| {
        trace!("Unspawning Minion: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    };

    for (mut minion, transform, health, entity) in &mut query {
        // edge of the world
        if transform.translation.x.abs() >= GAME_MAX_WIDTH / 2.
            || transform.translation.y.abs() >= GAME_MAX_HEIGHT / 2.
        {
            kill(entity);
        }

        // too old
        if minion.destroy_timer.tick(time.delta()).just_finished() {
            kill(entity);
        }

        // just not enough health
        if health.is_dead() {
            kill(entity);
        }
    }
}

// maybe this is a bad idea to have a system per component since the collision event is having all contacts
// it makes us loop inside collision events multiple time
fn check_collisions_players(
    mut query_players: Query<(Entity, &Team, &mut Health), With<crate::player::Player>>,
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
                player.2.hit(1.);

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

// maybe this is a bad idea to have a system per component since the collision event is having all contacts
// it makes us loop inside collision events multiple time
fn check_collisions_minions(
    mut query_minions: Query<(&Team, &mut Health), With<Minion>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let [(team_a, mut health_a), (team_b, mut health_b)] =
                    match query_minions.get_many_mut([*e1, *e2]) {
                        Err(_) => continue,
                        Ok(m) => m,
                    };

                // if they are from the same team, do nothing special
                if team_a.id == team_b.id {
                    continue;
                }

                // hurt each other
                health_a.hit(1.);
                health_b.hit(1.);
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn decay_life(time: Res<Time>, mut query_minions: Query<&mut Health, With<Minion>>) {
    for mut health in &mut query_minions {
        health.hit(DECAY_VALUE_PER_SEC * time.delta_seconds());
    }
}
