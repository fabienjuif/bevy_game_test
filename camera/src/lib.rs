use bevy::{
    app::{PostStartup, Update},
    asset::Assets,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Res, ResMut},
    },
    gizmos::{gizmos::Gizmos, GizmoConfig},
    math::{Vec2, Vec3},
    prelude::{
        default, App, Camera2dBundle, Entity, Plugin, PostUpdate, Query, Transform, With, Without,
    },
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::Time,
};

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Camera {
    target: Entity, // TODO: find a way to have multiple targets per camera, but also being able to have multi cameras (n-n)
    dead_zone: Vec2,
    target_prev_translation: Vec3,
    // look at this position, this is the player + velocity + factor
    // it allow us to place the camera a bit ahead of time
    look_at: Vec3,
}

impl Camera {
    pub fn new(target: Entity, dead_zone: Vec2) -> Self {
        Self {
            target,
            dead_zone,
            target_prev_translation: Vec3::ZERO,
            look_at: Vec3::ZERO,
        }
    }

    pub fn new_default(target: Entity) -> Self {
        Self {
            target,
            dead_zone: Vec2::new(30.0, 15.0),
            target_prev_translation: Vec3::ZERO,
            look_at: Vec3::ZERO,
        }
    }
}

#[derive(Bundle)]
pub struct CameraBundle {
    camera: Camera,
    bundle: Camera2dBundle,
}

impl CameraBundle {
    pub fn new(target: Entity, bundle: Camera2dBundle) -> Self {
        Self {
            camera: Camera::new_default(target),
            bundle,
        }
    }

    pub fn new_with_default_bundle(target: Entity) -> Self {
        Self {
            camera: Camera::new_default(target),
            bundle: Camera2dBundle::default(),
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, center)
            .add_systems(PostUpdate, cameraman);
    }
}

fn center(
    mut query_camera: Query<(&mut Transform, &mut Camera), Without<Target>>,
    query_targets: Query<(&Transform, Entity), With<Target>>,
) {
    for (mut camera_transform, mut camera) in &mut query_camera {
        for (target_transform, target_entity) in &query_targets {
            if camera.target != target_entity {
                continue;
            }

            // TODO: for now we follow the first target but we could think of doing an average positions of all the targets
            if camera.target == target_entity {
                camera_transform.translation.x = target_transform.translation.x;
                camera_transform.translation.y = target_transform.translation.y;
                camera.target_prev_translation = target_transform.translation;
                camera.look_at = target_transform.translation;
                break;
            }
        }
    }
}

fn cameraman(
    mut query_camera: Query<(&mut Transform, &mut Camera), Without<Target>>,
    query_targets: Query<(&Transform, Entity), With<Target>>,
    time: Res<Time>,
) {
    for (mut camera_transform, mut camera) in &mut query_camera {
        for (target_transform, target_entity) in &query_targets {
            if camera.target != target_entity {
                continue;
            }

            // process velocity
            let target_velocity = (target_transform.translation - camera.target_prev_translation)
                / time.delta().as_secs_f32();
            camera.look_at = target_transform.translation + target_velocity;
            camera.target_prev_translation = target_transform.translation;
            // TODO: DO SOMETHING ABOUT IT

            // TODO: for now we follow the first target but we could think of doing an average positions of all the targets
            if camera.target == target_entity {
                let diff = camera_transform.translation - target_transform.translation;
                let diff_abs = diff.abs();

                if diff_abs.x > camera.dead_zone.x {
                    camera_transform.translation.x = target_transform.translation.x
                        - if diff.x > 0. {
                            -camera.dead_zone.x
                        } else {
                            camera.dead_zone.x
                        };
                }
                if diff_abs.y > camera.dead_zone.y {
                    camera_transform.translation.y = target_transform.translation.y
                        - if diff.y > 0. {
                            -camera.dead_zone.y
                        } else {
                            camera.dead_zone.y
                        };
                }

                break;
            }
        }
    }
}

pub struct CameraDebugPlugin;

impl Plugin for CameraDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_debug)
            .add_systems(Update, debug);
    }
}

#[derive(Component)]
pub struct CameraDebug(Entity);

fn setup_debug(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query_cameras: Query<(&Transform, &Camera, Entity)>,
) {
    for (camera_transform, _camera, entity) in &query_cameras {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_translation(Vec3::new(
                    camera_transform.translation.x,
                    camera_transform.translation.y,
                    100.0,
                )),
                ..default()
            },
            CameraDebug(entity),
        ));
    }
}

#[allow(clippy::type_complexity)] // Because of query_targets
fn debug(
    mut gizmos: Gizmos,
    mut config: ResMut<GizmoConfig>,
    query_cameras: Query<(&Transform, &Camera, Entity)>,
    query_targets: Query<(&Transform, Entity), (With<Target>, Without<CameraDebug>)>,
    mut query_camera_debug: Query<(&mut Transform, &CameraDebug), Without<Camera>>,
) {
    config.line_width = 1.0;

    // TODO: Unspawn camera debug object if camera do not exist anymore

    for (camera_transform, camera, entity) in &query_cameras {
        gizmos.rect_2d(
            camera_transform.translation.truncate(),
            0.,
            camera.dead_zone * 2.0,
            Color::RED,
        );

        for (target_transform, target_entity) in &query_targets {
            if camera.target != target_entity {
                continue;
            }

            gizmos.line_2d(
                target_transform.translation.truncate(),
                camera.look_at.truncate(),
                Color::GREEN,
            );
        }

        for (mut transform, camera_debug) in &mut query_camera_debug {
            if camera_debug.0 != entity {
                continue;
            }
            transform.translation.x = camera.look_at.x;
            transform.translation.y = camera.look_at.y;
        }
    }
}
