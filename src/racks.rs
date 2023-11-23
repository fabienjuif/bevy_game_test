use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
};
use bevy_rapier2d::prelude::*;

use crate::{
    common::Rewards,
    health::Health,
    minions::MinionBundle,
    teams::{Team, Teams},
};

use rand::Rng;

pub const RACK_GOLD_VALUE: f32 = 10.;

#[derive(Component)]
pub struct Rack {
    pub minion_spawn_timer: Timer,
    pub minion_spawn_timer_q: Timer,
    pub minion_spawn_count: u32,
    pub minion_spawned_count: u32,
    pub minion_spawning: bool,
}

#[derive(Bundle)]
pub struct RackBundle {
    pub sprite_bundle: SpriteBundle,
    pub team: Team,
    pub rack: Rack,
    pub health: Health,
    pub rewards: Rewards,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub events: ActiveEvents,
    pub mass: ColliderMassProperties,
}

impl RackBundle {
    pub fn new(team: Team, transform: Transform) -> Self {
        let size = Vec2::new(20.0, 20.0);
        let mut minion_spawn_timer = Timer::from_seconds(1.5, TimerMode::Repeating);
        minion_spawn_timer.set_elapsed(Duration::from_secs_f32(1.0));
        RackBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: team.color,
                    custom_size: Some(size),
                    ..default()
                },
                transform,
                ..default()
            },
            team,
            rack: Rack {
                minion_spawning: false,
                minion_spawned_count: 0,
                minion_spawn_count: 5,
                minion_spawn_timer,
                minion_spawn_timer_q: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            health: Health::new(220.)
                .with_health_bar_position(Vec3::new(0.0, 20.0, 0.0))
                .with_health_bar_size(Vec2::new(size.x, 5.)),
            rewards: Rewards { gold: 100. },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid((size.x / 2.) * 0.98, (size.y / 2.) * 0.98),
            events: ActiveEvents::COLLISION_EVENTS,
            mass: ColliderMassProperties::Mass(0.),
        }
    }
}

pub struct RacksPlugin;

impl Plugin for RacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, spawn_minions)
            .add_systems(PostUpdate, destroy);
    }
}

fn setup(mut commands: Commands, teams: Res<Teams>) {
    commands.spawn(RackBundle::new(
        teams.get_expect("b".into()),
        Transform::from_xyz(200.0, 200.0, 0.),
    ));
    commands.spawn(RackBundle::new(
        teams.get_expect("b".into()),
        Transform::from_xyz(250.0, 150.0, 0.),
    ));
    commands.spawn(RackBundle::new(
        teams.get_expect("c".into()),
        Transform::from_xyz(100.0, -100.0, 0.),
    ));
}

fn spawn_minions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Rack, &Collider, &Transform, &Team)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for (mut rack, collider, transform, team) in &mut query {
        // ticks timers
        rack.minion_spawn_timer_q.tick(time.delta());
        rack.minion_spawn_timer.tick(time.delta());

        // we are ready to start spawning
        if rack.minion_spawn_timer.just_finished() {
            debug!("[rack] ready to spawn minions!");
            rack.minion_spawned_count = 0;
            rack.minion_spawning = true;
        }

        // we are actually spawning, gogogogogo
        // but please one at the time for physics engine reasons
        if rack.minion_spawning
            && rack.minion_spawn_timer_q.just_finished()
            && rack.minion_spawned_count < rack.minion_spawn_count
        {
            // TODO: should RNG an angle instead
            if let Some(cuboid) = collider.as_cuboid() {
                let mut offset_x = cuboid.half_extents().x + rng.gen_range(2.0..10.0);
                let mut offset_y = cuboid.half_extents().y + rng.gen_range(2.0..10.0);

                if rng.gen_bool(0.5) {
                    offset_x *= -1.;
                }
                if rng.gen_bool(0.5) {
                    offset_y *= -1.;
                }

                commands.spawn(MinionBundle::new(
                    &mut meshes,
                    &mut materials,
                    Vec3::new(
                        transform.translation.x + offset_x,
                        transform.translation.y + offset_y,
                        transform.translation.z,
                    ),
                    team.clone(),
                ));
                rack.minion_spawned_count += 1;

                if rack.minion_spawned_count >= rack.minion_spawn_count {
                    debug!("[rack] every minions are spawned!");
                    rack.minion_spawning = false;
                }
            }
        }
    }
}

// TODO: maybe this system can be retrieve from health bar crate (give the type and insert it in the filter?)
fn destroy(mut commands: Commands, mut query: Query<(&Health, Entity), With<Rack>>) {
    let mut kill = |entity| {
        trace!("Unspawning Minion: {:?}", entity);
        commands.entity(entity).despawn_recursive();
    };

    for (health, entity) in &mut query {
        // just not enough health
        if health.is_dead() {
            kill(entity);
        }
    }
}
