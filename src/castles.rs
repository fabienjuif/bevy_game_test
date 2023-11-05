use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::{Timer, TimerMode},
};
use bevy_rapier2d::prelude::*;

use crate::{
    common::Rewards,
    health::Health,
    racks::Rack,
    teams::{Team, Teams},
};

// TODO: Castle is a rack two...
// TODO: So maybe having a common "minion spawner" rather than "rack" is better?

#[derive(Bundle)]
pub struct CastleBundle {
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

impl CastleBundle {
    pub fn new(team: Team, transform: Transform) -> Self {
        let size = Vec2::new(80.0, 80.0);
        CastleBundle {
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
                minion_spawn_timer: Timer::from_seconds(3., TimerMode::Repeating),
                minion_spawn_timer_q: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
            health: Health::new(300.)
                .with_health_bar_position(Vec3::new(0.0, 50.0, 0.0))
                .with_health_bar_size(Vec2::new(size.x, 5.)),
            rewards: Rewards { gold: 500. },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            events: ActiveEvents::COLLISION_EVENTS,
            mass: ColliderMassProperties::Mass(0.),
        }
    }
}

pub struct CastlesPlugin;

impl Plugin for CastlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, destroy);
    }
}

fn setup(mut commands: Commands, teams: Res<Teams>) {
    commands.spawn(CastleBundle::new(
        teams.get_expect("a".into()),
        Transform::from_xyz(-200.0, -300.0, 0.),
    ));
    commands.spawn(CastleBundle::new(
        teams.get_expect("b".into()),
        Transform::from_xyz(300.0, 300.0, 0.),
    ));
    commands.spawn(CastleBundle::new(
        teams.get_expect("c".into()),
        Transform::from_xyz(200.0, -200.0, 0.),
    ));
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
