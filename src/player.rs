use crate::common::*;
use crate::health_bar::{Health, HealthBarBundle};
use crate::racks::{RackBundle, RACK_GOLD_VALUE};
use crate::teams::{Team, Teams};
use bevy::{
    input::gamepad::GamepadButtonChangedEvent,
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::Time,
};
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
struct LocalPlayer;

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
            collider: Collider::cuboid(50.0, 25.),
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
        app.add_systems(Startup, (setup, setup_ui)).add_systems(
            Update,
            (
                update_axes,
                update_button_values,
                check_collisions_sword,
                update_ui,
                update_sword,
                update_cooldowns,
            ),
        );
    }
}

fn setup(mut commands: Commands, teams: Res<Teams>) {
    let mut sword_cooldown = Timer::from_seconds(0.3, TimerMode::Once);
    sword_cooldown.set_elapsed(sword_cooldown.duration());

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
            RigidBody::KinematicVelocityBased,
            // RigidBody::Dynamic,
            Collider::cuboid(25.0, 25.),
            ActiveEvents::COLLISION_EVENTS,
            LocalPlayer {},
            Player {
                gold: 20.,
                cooldowns: Cooldowns {
                    sword: sword_cooldown,
                },
            },
            Health::new(100.),
            Name("local_player".to_string()),
            teams.get_expect("a".into()),
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

    commands.spawn(HealthBarBundle::new(
        entity,
        Vec3::new(0.0, 40.0, 0.1),
        Vec2::new(50.0, 5.0),
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
    for button_event in events.iter() {
        let (mut player, transform, team, entity, children) = query_local_player.single_mut();
        if button_event.button_type == GamepadButtonType::South {
            for child in children {
                if let Ok(mut sprite) = query.get_mut(*child) {
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
    // mut commands: Commands,
    query_swords: Query<(Entity, &Sword)>,
    mut query_player: Query<(&mut Player, &Team)>,
    mut query_hit_entities: Query<(&Rewards, &Team, &mut Health)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
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
                let (rewards, hit_team, mut health) = match health {
                    None => continue,
                    Some(o) => o,
                };

                // hurt
                health.hit(20.);

                // player attached to this sword receive gold
                if let Ok((mut player, team)) = query_player.get_mut(sword.entity) {
                    if team.id != hit_team.id {
                        player.gold += rewards.gold;
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
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
