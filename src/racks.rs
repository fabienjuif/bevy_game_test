use bevy::{
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
};

use crate::teams::{Team, Teams};

pub const RACK_GOLD_VALUE: f32 = 10.;

#[derive(Component)]
pub struct Rack {
    minion_spawn_timer: Timer,
    minion_spawn_timer_q: Timer,
    minion_spawn_count: u32,
    minion_spawned_count: u32,
    minion_spawning: bool,
}

#[derive(Bundle)]
pub struct RackBundle {
    pub sprite_bundle: SpriteBundle,
    pub team: Team,
    pub rack: Rack,
}

impl RackBundle {
    pub fn new(team: Team, transform: Transform) -> Self {
        RackBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: team.color,
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform,
                ..default()
            },
            team,
            rack: Rack {
                minion_spawning: false,
                minion_spawned_count: 0,
                minion_spawn_count: 10,
                minion_spawn_timer: Timer::from_seconds(3., TimerMode::Repeating),
                minion_spawn_timer_q: Timer::from_seconds(0.2, TimerMode::Repeating),
            },
        }
    }
}

pub struct RacksPlugin;

impl Plugin for RacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rack)
            .add_systems(Update, spawn_minions);
    }
}

fn setup_rack(mut commands: Commands, teams: Res<Teams>) {
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
    mut query: Query<(&mut Rack, &Transform, &Team)>,
) {
    for (mut rack, transform, team) in &mut query {
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
            crate::minions::spawn_minion(&mut commands, transform, team.clone());
            rack.minion_spawned_count += 1;

            if rack.minion_spawned_count >= rack.minion_spawn_count {
                debug!("[rack] every minions are spawned!");
                rack.minion_spawning = false;
            }
        }
    }
}
