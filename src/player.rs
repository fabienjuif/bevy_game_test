use crate::common::*;
use crate::health_bar::{Health, HealthBarBundle};
use crate::racks::RACK_GOLD_VALUE;
use bevy::{
    input::gamepad::GamepadButtonChangedEvent,
    prelude::*,
    sprite::{Sprite, SpriteBundle},
    time::Time,
};
use bevy_rapier2d::prelude::*;

const DEFAULT_HAND_COLOR: Color = Color::rgb(0.8, 0.25, 0.24);
const JOYSTICK_SCALE: f32 = 200.;

#[derive(Component)]
pub struct Player {
    pub gold: u32,
}

#[derive(Component)]
struct LocalPlayer;

#[derive(Component)]
struct Hand;

#[derive(Component)]
struct GoldUI;

#[derive(Component)]
struct Sword;

impl Sword {
    pub fn collider() -> Collider {
        Collider::cuboid(50.0, 25.)
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
            ),
        );
    }
}

fn setup(mut commands: Commands) {
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
            ActiveEvents::COLLISION_EVENTS,
            LocalPlayer {},
            Player { gold: 20 },
            Health::new(100.),
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

            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.25, 0.6, 0.25),
                        custom_size: Some(Vec2::new(100.0, 50.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 35.0, 10.0),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Sensor,
                Sword,
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
    mut query_local_player: Query<(&mut Player, &Transform, &Team, &Children), With<LocalPlayer>>,
    mut query: Query<&mut Sprite, With<Hand>>,
    query_sword: Query<Entity, With<Sword>>,
) {
    for button_event in events.iter() {
        if button_event.button_type == GamepadButtonType::South {
            for (_, _, _, children) in &query_local_player {
                for child in children {
                    if let Ok(mut sprite) = query.get_mut(*child) {
                        if button_event.value != 0. {
                            sprite.color = Color::rgb(0.25, 0.75, 0.25)
                        } else {
                            sprite.color = DEFAULT_HAND_COLOR
                        }
                    }

                    if let Ok(entity) = query_sword.get(*child) {
                        if button_event.value != 0. {
                            commands
                                .entity(entity)
                                .insert((Sword::collider(), Visibility::Visible));
                        } else {
                            commands
                                .entity(entity)
                                .remove::<Collider>()
                                .insert(Visibility::Hidden);
                        }
                    }
                }
            }
        }

        if button_event.button_type == GamepadButtonType::East && button_event.value != 0. {
            let (mut player, transform, team, _) = query_local_player.single_mut();
            if player.gold >= RACK_GOLD_VALUE {
                crate::racks::spawn_rack(&mut commands, *transform, team.clone());
                player.gold -= RACK_GOLD_VALUE;
            }
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
    query_swords: Query<Entity, With<Sword>>,
    mut query_health: Query<(Entity, &mut Health)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if query_swords
                    .get(*e1)
                    .ok()
                    .or_else(|| query_swords.get(*e2).ok())
                    .is_none()
                {
                    continue;
                }

                let health = if query_health.contains(*e1) {
                    query_health.get_mut(*e1).ok()
                } else {
                    query_health.get_mut(*e2).ok()
                };
                let (_, mut health) = match health {
                    None => continue,
                    Some(o) => o,
                };

                // hurt
                // TODO: health should have a function to remove some offset
                health.value -= 20.;
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}
