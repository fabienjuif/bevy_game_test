mod doc_plugin;

use bevy::{
    input::gamepad::GamepadButtonChangedEvent,
    log::{Level, LogPlugin},
    prelude::*,
};
use doc_plugin::HelloPlugin;
use rand::Rng;

const JOYSTICK_SCALE: f32 = 200.;
const MINION_SCALE: f32 = 100.;
const DEFAULT_HAND_COLOR: Color = Color::rgb(0.8, 0.25, 0.24);
const GAME_MAX_WIDTH: f32 = 2000.;
const GAME_MAX_HEIGHT: f32 = 2000.;

// TODO: Move minions into a plugin
// TODO: Move local player into a plugin

#[derive(Component)]
struct LocalPlayer;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Hand;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Minion;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Target {
    position: Vec3,
}

#[derive(Component)]
struct MinionsSpawner {
    timer: Timer,
    count: u32,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::TRACE,
                filter:
                    "wgpu=error,bevy_render=warn,bevy_app=warn,bevy_ecs=warn,naga=warn,gilrs=warn"
                        .to_string(),
            }),
            HelloPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_axes,
                update_button_values,
                update_move_minions,
                spawn_minions,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), Camera {}));

    // Spawner
    commands.spawn(MinionsSpawner {
        timer: Timer::from_seconds(3., TimerMode::Repeating),
        count: 50,
    });

    // Local player
    commands
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
            LocalPlayer {},
            Name("local_player".to_string()),
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
        });
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
    mut parents_query: Query<&Children, With<LocalPlayer>>,
    mut query: Query<&mut Sprite, With<Hand>>,
) {
    for button_event in events.iter() {
        if button_event.button_type == GamepadButtonType::South {
            for children in &mut parents_query {
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
            spawn_minion(&mut commands)
        }
    }
}

// TODO: Use global tranform?
// TODO: use a seed random maybe Bevy has one
// TODO: Target should be updated in the main loop (and not being randomize)
fn spawn_minion(commands: &mut Commands) {
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

fn spawn_minions(mut commands: Commands, time: Res<Time>, mut query: Query<&mut MinionsSpawner>) {
    for mut spawner in &mut query {
        if spawner.timer.tick(time.delta()).just_finished() {
            for _ in 0..spawner.count {
                spawn_minion(&mut commands);
            }
        }
    }
}
