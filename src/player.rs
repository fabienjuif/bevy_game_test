use crate::common::*;
use crate::health::Health;
use crate::physics::CollisionEvent;
use crate::racks::{RackBundle, RACK_GOLD_VALUE};
use crate::states::GameState;
use crate::teams::{Team, Teams};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use bevy::{
    input::gamepad::GamepadButtonChangedEvent,
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::Time,
};
use bevy_cameraman::{CameraBundle, Cameraman, Target};
use bevy_rapier2d::prelude::*;

const DEFAULT_HAND_COLOR: Color = Color::rgb(0.8, 0.25, 0.24);
const JOYSTICK_SCALE: f32 = 200.;

pub struct Cooldowns {
    pub sword: Timer,
}

#[derive(Component)]
pub struct Player {
    pub gold: f32,
    pub cooldowns: Cooldowns,
}

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
struct Hand;

#[derive(Component)]
struct GoldUI;

#[derive(Component)]
struct Sword {
    entity: Entity,
    duration: Timer,
}

#[derive(Bundle)]
struct SwordBundle {
    pub sprite: SpriteBundle,
    pub sword: Sword,
    pub sensor: Sensor,
    pub collider: Collider,
}

impl SwordBundle {
    pub fn new(parent_entity: Entity) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.6, 0.25),
                    custom_size: Some(Vec2::new(100.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 35.0, 10.0),
                ..default()
            },
            collider: Collider::cuboid(49.0, 24.),
            sensor: Sensor,
            sword: Sword {
                entity: parent_entity,
                duration: Timer::from_seconds(0.2, TimerMode::Once),
            },
        }
    }
}

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (setup, setup_ui))
            .add_systems(
                Update,
                (
                    // movements
                    update_axes,
                    keyboard_movements,
                    mouse_movements,
                    // actions
                    update_button_values,
                    mouse_actions,
                    keyboard_actions,
                    // others
                    check_collisions_sword,
                    update_ui,
                    update_sword,
                    update_cooldowns,
                )
                    .run_if(in_state(GameState::Game)),
            );
    }
}

fn setup(
    mut commands: Commands,
    teams: Res<Teams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut sword_cooldown = Timer::from_seconds(0.3, TimerMode::Once);
    sword_cooldown.set_elapsed(sword_cooldown.duration());

    let team = teams.get_expect("a".into());

    let entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(30.).into()).into(),
                material: materials.add(ColorMaterial::from(team.color)),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            },
            // TODO:: add sprite sheet later
            // SpriteBundle {
            //     sprite: Sprite {
            //         color: Color::rgb(0.25, 0.25, 0.75),
            //         custom_size: Some(Vec2::new(50.0, 50.0)),
            //         ..default()
            //     },
            //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
            //     ..default()
            // },
            RigidBody::KinematicVelocityBased,
            // RigidBody::Dynamic,
            Collider::ball(28.),
            LocalPlayer {},
            Player {
                gold: 20.,
                cooldowns: Cooldowns {
                    sword: sword_cooldown,
                },
            },
            Health::new(100.)
                .with_health_bar_position(Vec3::new(0.0, 40.0, 0.1))
                .with_health_bar_size(Vec2::new(50.0, 5.0)),
            Name("local_player".to_string()),
            Target,
            team,
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

    commands.spawn(CameraBundle::new(
        Cameraman::new(entity, Vec2::new(50.0, 20.0), Vec3::ONE * 0.8),
        Camera2dBundle::default(),
    ));
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            margin: UiRect::all(Val::Px(5.)),
            ..default()
        }),
        // Because this is a distinct label widget and
        // not button/list item text, this is necessary
        // for accessibility to treat the text accordingly.
        Label,
        GoldUI,
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
        if left_stick_x.abs() > 0.1 {
            moved = true;
            for mut transform in &mut query {
                transform.translation.x += left_stick_x * JOYSTICK_SCALE * time.delta_seconds();
            }
        }

        let left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.1 {
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
    mut query_local_player: Query<
        (&mut Player, &Transform, &Team, Entity, &Children),
        With<LocalPlayer>,
    >,
    mut query: Query<&mut Sprite, With<Hand>>,
) {
    for button_event in events.read() {
        let (mut player, transform, team, entity, children) = query_local_player.single_mut();
        if button_event.button_type == GamepadButtonType::South {
            for child in children {
                if let Ok(mut sprite) = query.get_mut(*child) {
                    // TODO: check how to send even in bevy to DRY on actions
                    if button_event.value != 0. {
                        if player.cooldowns.sword.finished() {
                            sprite.color = Color::rgb(0.25, 0.75, 0.25);
                            let sword_entity = commands.spawn(SwordBundle::new(entity)).id();
                            commands.entity(entity).add_child(sword_entity);
                            player.cooldowns.sword.reset();
                        }
                    } else {
                        sprite.color = DEFAULT_HAND_COLOR;
                    }
                }
            }
        }

        // TODO: check how to send even in bevy to DRY on actions
        if button_event.button_type == GamepadButtonType::East
            && button_event.value != 0.
            && player.gold >= RACK_GOLD_VALUE
        {
            commands.spawn(RackBundle::new(team.clone(), *transform));
            player.gold -= RACK_GOLD_VALUE;
        }
    }
}

fn update_ui(
    query_player: Query<&Player, With<LocalPlayer>>,
    mut query_ui: Query<&mut Text, With<GoldUI>>,
) {
    let player = query_player.get_single().expect("no player found");
    let mut text = query_ui.get_single_mut().expect("no gold ui found");

    text.sections[0].value = format!("Gold: {}", player.gold);
}

// maybe this is a bad idea to have a system per component since the collision event is having all contacts
// it makes us loop inside collision events multiple time
fn check_collisions_sword(
    query_swords: Query<(Entity, &Sword)>,
    mut query_player: Query<(&mut Player, &Team)>,
    mut query_hit_entities: Query<(Option<&Rewards>, &Team, &mut Health)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2) => {
                let sword = query_swords
                    .get(*e1)
                    .ok()
                    .or_else(|| query_swords.get(*e2).ok());
                let (_, sword) = match sword {
                    None => continue,
                    Some(o) => o,
                };

                let health = if query_hit_entities.contains(*e1) {
                    query_hit_entities.get_mut(*e1).ok()
                } else {
                    query_hit_entities.get_mut(*e2).ok()
                };
                let (rewards_opt, hit_team, mut health) = match health {
                    None => continue,
                    Some(o) => o,
                };

                // hurt
                if health.hit(20.).is_dead() {
                    // player attached to this sword receive gold
                    if let Ok((mut player, team)) = query_player.get_mut(sword.entity) {
                        if let Some(rewards) = rewards_opt {
                            if team.id != hit_team.id {
                                player.gold += rewards.gold;
                            }
                        }
                    }
                }
            }
            CollisionEvent::Stopped(_, _) => {}
        }
    }
}

fn update_sword(
    mut commands: Commands,
    time: Res<Time>,
    mut query_swords: Query<(Entity, &mut Sword)>,
) {
    for (entity, mut sword) in query_swords.iter_mut() {
        if sword.duration.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_cooldowns(time: Res<Time>, mut query_players: Query<&mut Player>) {
    for mut player in query_players.iter_mut() {
        player.cooldowns.sword.tick(time.delta());
    }
}

fn keyboard_movements(
    keyboard_input: Res<Input<KeyCode>>,
    mut query_player: Query<&mut Transform, With<LocalPlayer>>,
    time: Res<Time>,
) {
    for mut transform in &mut query_player {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            transform.translation.x -= JOYSTICK_SCALE * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            transform.translation.x += JOYSTICK_SCALE * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            transform.translation.y -= JOYSTICK_SCALE * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            transform.translation.y += JOYSTICK_SCALE * time.delta_seconds();
        }
    }
}

fn mouse_movements(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut query_player: Query<&mut Transform, With<LocalPlayer>>,
    query_camera: Query<&Transform, (Without<LocalPlayer>, With<Camera>)>,
) {
    let Ok(window) = primary_window.get_single() else {
        return;
    };
    let Ok(camera_transform) = query_camera.get_single() else {
        return;
    };

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(mut player_transform) = query_player.get_single_mut() {
            let window_half_size = Vec2::new(window.width(), window.height()) / 2.;
            let cursor_position = Vec2::new(
                cursor_position.x - window_half_size.x + camera_transform.translation.x,
                -cursor_position.y + window_half_size.y + camera_transform.translation.y,
            );
            let pos = player_transform.translation.truncate(); // player position

            let direction = cursor_position - pos;
            player_transform.rotation = Quat::from_rotation_z((-direction.x).atan2(direction.y));
        }
    }
}

fn mouse_actions(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut query_local_player: Query<(&mut Player, Entity, &Children), With<LocalPlayer>>,
    mut query: Query<&mut Sprite, With<Hand>>,
) {
    let (mut player, entity, children) = query_local_player.single_mut();

    for child in children {
        if let Ok(mut sprite) = query.get_mut(*child) {
            // TODO: check how to send even in bevy to DRY on actions
            if buttons.just_pressed(MouseButton::Left) {
                if player.cooldowns.sword.finished() {
                    sprite.color = Color::rgb(0.25, 0.75, 0.25);
                    let sword_entity = commands.spawn(SwordBundle::new(entity)).id();
                    commands.entity(entity).add_child(sword_entity);
                    player.cooldowns.sword.reset();
                }
            } else {
                sprite.color = DEFAULT_HAND_COLOR;
            }
        }
    }
}

fn keyboard_actions(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query_local_player: Query<(&mut Player, &Transform, &Team), With<LocalPlayer>>,
) {
    let (mut player, transform, team) = query_local_player.single_mut();

    if keyboard_input.just_pressed(KeyCode::E) && player.gold >= RACK_GOLD_VALUE {
        commands.spawn(RackBundle::new(team.clone(), *transform));
        player.gold -= RACK_GOLD_VALUE;
    }
}
