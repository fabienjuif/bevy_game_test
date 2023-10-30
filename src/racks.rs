use bevy::{
    prelude::{
        default, trace, App, Color, Commands, Component, Plugin, Query, Res, Startup, Transform,
        Update, Vec2,
    },
    sprite::{Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
};

use crate::common::Team;

#[derive(Component)]
pub struct Rack {
    minion_spawn_timer: Timer,
    minion_spawn_count: u32,
}

pub struct RacksPlugin;

impl Plugin for RacksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rack)
            .add_systems(Update, spawn_minions);
    }
}

fn setup_rack(mut commands: Commands) {
    spawn_rack(
        &mut commands,
        Transform::from_xyz(200.0, 200.0, 0.),
        Team {
            id: "b".to_string(),
            color: Color::rgb(0.8, 0.3, 0.3),
        },
    )
}

fn spawn_minions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Rack, &Transform, &Team)>,
) {
    for (mut rack, transform, team) in &mut query {
        if rack.minion_spawn_timer.tick(time.delta()).just_finished() {
            for _ in 0..rack.minion_spawn_count {
                crate::minions::spawn_minion(&mut commands, transform, team.clone());
            }
        }
    }
}

// TODO: Use global tranform?
pub fn spawn_rack(commands: &mut Commands, transform: Transform, team: Team) {
    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: team.color,
                    custom_size: Some(Vec2::new(20.0, 20.0)),
                    ..default()
                },
                transform,
                ..default()
            },
            team,
            Rack {
                minion_spawn_count: 10,
                minion_spawn_timer: Timer::from_seconds(3., TimerMode::Repeating),
            },
        ))
        .id();

    trace!("Spawning Rack: {:?}", id);
}
