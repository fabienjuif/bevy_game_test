mod common;
mod health_bar;
mod minions;
mod racks;

use bevy::{
    input::gamepad::GamepadButtonChangedEvent,
    log::{Level, LogPlugin},
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    DefaultPlugins,
};
use bevy_rapier2d::{prelude::*, render::RapierDebugRenderPlugin};
use common::*;
use health_bar::{Health, HealthBar, HealthBarPlugin};
use minions::MinionsPlugin;
use racks::RacksPlugin;

const JOYSTICK_SCALE: f32 = 200.;
const DEFAULT_HAND_COLOR: Color = Color::rgb(0.8, 0.25, 0.24);

// TODO: Move local player into a plugin

#[derive(Component)]
struct LocalPlayer;

#[derive(Component)]
struct Hand;

#[derive(Component)]
struct Camera;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(LogPlugin {
            level: Level::TRACE,
            filter: "wgpu=error,bevy_render=warn,bevy_app=warn,bevy_ecs=warn,naga=warn,gilrs=warn"
                .to_string(),
        }),
        MinionsPlugin,
        RacksPlugin,
        HealthBarPlugin,
    ));

    init_physics(&mut app);

    app.add_systems(Startup, setup)
        .add_systems(Update, (update_axes, update_button_values));

    app.run();
}

fn init_physics(app: &mut App) {
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        RapierDebugRenderPlugin::default(),
    ))
    // removes gravity
    .insert_resource(RapierConfiguration {
        gravity: Vec2::new(0.0, 0.0),
        ..default()
    });
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), Camera {}));

    // Local player
    let entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            // RigidBody::KinematicVelocityBased,
            RigidBody::Dynamic,
            Collider::cuboid(25.0, 25.),
            LocalPlayer {},
            Health { health: 100. },
            Name("local_player".to_string()),
            Team {
                id: "a".to_string(),
                color: Color::rgb(0.3, 0.3, 0.8),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: DEFAULT_HAND_COLOR,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 22.0, 0.1),
                    ..default()
                },
                Hand {},
            ));
        })
        .id();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: DEFAULT_HEALTH_COLOR,
                custom_size: Some(Vec2::new(50.0, 5.0)),
                ..default()
            },
            ..default()
        },
        HealthBar {
            entity,
            translation: Vec3::new(0.0, 40.0, 0.1),
        },
    ));
}

fn update_axes(
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<&mut Transform, (With<LocalPlayer>, With<Children>)>,
) {
    // TODO: Affect one gamepad to local player
    for gamepad in gamepads.iter() {
        let left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        let mut moved = false;
        if left_stick_x.abs() > 0.01 {
            moved = true;
            for mut transform in &mut query {
                transform.translation.x += left_stick_x * JOYSTICK_SCALE * time.delta_seconds();
            }
        }

        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.01 {
            moved = true;
            for mut transform in &mut query {
                transform.translation.y += left_stick_y * JOYSTICK_SCALE * time.delta_seconds();
            }
        }

        if moved {
            for mut transform in &mut query {
                transform.rotation = Quat::from_axis_angle(
                    Vec3::new(0., 0., 1.),
                    (-left_stick_x).atan2(left_stick_y),
                );
            }
        }
    }
}

fn update_button_values(
    mut commands: Commands,
    mut events: EventReader<GamepadButtonChangedEvent>,
    query_local_player: Query<(&Transform, &Team, &Children), With<LocalPlayer>>,
    mut query: Query<&mut Sprite, With<Hand>>,
) {
    for button_event in events.iter() {
        if button_event.button_type == GamepadButtonType::South {
            for (_, _, children) in &query_local_player {
                for child in children {
                    if let Ok(mut sprite) = query.get_mut(*child) {
                        if button_event.value != 0. {
                            sprite.color = Color::rgb(0.25, 0.75, 0.25)
                        } else {
                            sprite.color = DEFAULT_HAND_COLOR
                        }
                    }
                }
            }
        }

        if button_event.button_type == GamepadButtonType::East && button_event.value != 0. {
            let (transform, team, _) = query_local_player.single();
            racks::spawn_rack(&mut commands, *transform, team.clone());
        }
    }
}
