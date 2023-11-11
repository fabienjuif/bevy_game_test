use crate::{common::*, health::Health, teams::Team};
use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    time::{Time, Timer},
    utils::{default, HashMap},
};
use bevy_rapier2d::prelude::*;

const MINION_SCALE: f32 = 190.;
const DESTROY_MINIONS_AFTER_SECS: f32 = 120.;
const DECAY_VALUE_PER_SEC: f32 = 10.;
const REWARDS_GOLD: f32 = 1.;

pub struct MinionsPlugin;

// TODO: move this into common
#[derive(Component)]
struct TimeDestroyable {
    timer: Timer,
}

#[derive(Component)]
struct Minion {
    had_exploded: bool,
}

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_move_minions, check_collisions_minions, decay_life),
        )
        .add_systems(PostUpdate, (destroy_minions, destroy_after_timer));
    }
}

#[derive(Bundle)]
pub struct MinionBundle {
    minion: Minion,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    // sprite: SpriteBundle,
    health: Health,
    rewards: Rewards,
    team: Team,

    // physics
    body: RigidBody,
    collider: Collider,
    events: ActiveEvents,
    timer_destroyable: TimeDestroyable,
}

impl MinionBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        translation: Vec3,
        team: Team,
    ) -> Self {
        MinionBundle {
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(6.).into()).into(),
                material: materials.add(ColorMaterial::from(team.color)),
                transform: Transform::from_translation(translation),
                ..default()
            },
            // TODO: add sprite sheet later
            // sprite: SpriteBundle {
            //     sprite: Sprite {
            //         color: team.color,
            //         custom_size: Some(Vec2::new(10.0, 10.0)),
            //         ..default()
            //     },
            //     transform: Transform::from_translation(translation),
            //     ..default()
            // },
            minion: Minion {
                had_exploded: false,
            },
            health: Health::new(20.)
                .with_health_bar_position(Vec3::new(0.0, 15.0, 0.1))
                .with_health_bar_size(Vec2::new(10.0, 5.0)),
            rewards: Rewards { gold: REWARDS_GOLD },
            team,
            // physics
            body: RigidBody::Dynamic,
            collider: Collider::ball(6.),
            events: ActiveEvents::COLLISION_EVENTS,
            timer_destroyable: TimeDestroyable {
                timer: Timer::from_seconds(DESTROY_MINIONS_AFTER_SECS, bevy::time::TimerMode::Once),
            },
        }
    }
}

#[derive(Component)]
struct Explosion;

#[derive(Bundle)]
struct ExplosionBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    explosion: Explosion,
    team: Team,
    sensor: Sensor,
    collider: Collider,
    timer_destroyable: TimeDestroyable,
}

impl ExplosionBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        translation: Vec3,
        team: Team,
    ) -> Self {
        let radius = 20.;
        ExplosionBundle {
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                material: materials.add(ColorMaterial::from(team.color)),
                transform: Transform::from_translation(translation),
                ..default()
            },
            explosion: Explosion,
            team,
            collider: Collider::ball(radius),
            sensor: Sensor,
            timer_destroyable: TimeDestroyable {
                timer: Timer::from_seconds(0.5, bevy::time::TimerMode::Once),
            },
        }
    }
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
    mut commands: Commands,
    query: Query<(&Transform, &Health, Entity), With<Minion>>,
) {
    let mut kill = |entity| {
        commands.entity(entity).despawn_recursive();
    };

    for (transform, health, entity) in query.iter() {
        // edge of the world
        if transform.translation.x.abs() >= GAME_MAX_WIDTH / 2.
            || transform.translation.y.abs() >= GAME_MAX_HEIGHT / 2.
        {
            kill(entity);
        }

        // just not enough health
        if health.is_dead() {
            kill(entity);
        }
    }
}

// TODO: move this into common plugin
fn destroy_after_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut TimeDestroyable, Entity)>,
) {
    for (mut time_destroyable, entity) in &mut query {
        if time_destroyable.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// maybe this is a bad idea to have a system per component since the collision event is having all contacts
// it makes us loop inside collision events multiple time
// TODO: We can have a more global system to handle that in one loop querying "atker" component (with a team),
// TODO: and putting reward in the team rather than in the player, or we want to stick of having reward on each entity so when a entity die its reward are huge?
// FIXME: With explosion implem, it can be simplify
fn check_collisions_minions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut collision_events: EventReader<CollisionEvent>,
    // queries
    mut query_minions: Query<(&Transform, &Team, &mut Minion), With<Minion>>,
    query_hit_entities: Query<&Team, Without<Minion>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                // between minions
                if query_minions.contains(*e1) && query_minions.contains(*e2) {
                    let [(transform_a, team_a, mut minion_a), (_, team_b, _)] =
                        match query_minions.get_many_mut([*e1, *e2]) {
                            Err(_) => continue,
                            Ok(m) => m,
                        };

                    // if minion a already exploded
                    if minion_a.had_exploded {
                        continue;
                    }

                    // if they are from the same team, do nothing special
                    if team_a.id == team_b.id {
                        continue;
                    }

                    commands.spawn(ExplosionBundle::new(
                        &mut meshes,
                        &mut materials,
                        transform_a.translation,
                        team_a.clone(),
                    ));

                    minion_a.had_exploded = true;

                    continue;
                }

                // minion vs others
                let (minion_transform, minion_team, mut minion) = match query_minions.get_mut(*e1) {
                    Err(_) => match query_minions.get_mut(*e2) {
                        Err(_) => continue,
                        Ok(value) => value,
                    },
                    Ok(value) => value,
                };

                let Ok(team) = query_hit_entities
                    .get(*e1)
                    .or_else(|_| query_hit_entities.get(*e2))
                else {
                    continue;
                };

                if minion.had_exploded {
                    continue;
                }

                // if they are from the same team, do nothing special
                if minion_team.id == team.id {
                    continue;
                }

                commands.spawn(ExplosionBundle::new(
                    &mut meshes,
                    &mut materials,
                    minion_transform.translation,
                    minion_team.clone(),
                ));

                minion.had_exploded = true;
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
