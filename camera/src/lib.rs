use bevy::{
    app::{PostStartup, Update},
    ecs::system::ResMut,
    gizmos::{gizmos::Gizmos, GizmoConfig},
    math::Vec2,
    prelude::{
        App, Bundle, Camera2dBundle, Component, Entity, Plugin, PostUpdate, Query, Transform, With,
        Without,
    },
    render::color::Color,
};

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Camera {
    target: Entity, // TODO: find a way to have multiple targets per camera, but also being able to have multi cameras (n-n)
    offset: Vec2,
}

impl Camera {
    pub fn new(target: Entity, offset: Vec2) -> Self {
        Self { target, offset }
    }

    pub fn new_default(target: Entity) -> Self {
        Self {
            target,
            offset: Vec2::new(100.0, 80.0),
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
    mut query_camera: Query<(&mut Transform, &Camera), Without<Target>>,
    query_targets: Query<(&Transform, Entity), With<Target>>,
) {
    for (mut camera_transform, camera) in &mut query_camera {
        for (target_transform, target_entity) in &query_targets {
            if camera.target != target_entity {
                continue;
            }

            // TODO: for now we follow the first target but we could think of doing an average positions of all the targets
            if camera.target == target_entity {
                camera_transform.translation.x = target_transform.translation.x;
                camera_transform.translation.y = target_transform.translation.y;
                break;
            }
        }
    }
}

fn cameraman(
    mut query_camera: Query<(&mut Transform, &Camera), Without<Target>>,
    query_targets: Query<(&Transform, Entity), With<Target>>,
) {
    for (mut camera_transform, camera) in &mut query_camera {
        for (target_transform, target_entity) in &query_targets {
            if camera.target != target_entity {
                continue;
            }

            // TODO: for now we follow the first target but we could think of doing an average positions of all the targets
            if camera.target == target_entity {
                let diff = camera_transform.translation - target_transform.translation;
                let diff_abs = diff.abs();

                if diff_abs.x > camera.offset.x {
                    camera_transform.translation.x = target_transform.translation.x
                        - if diff.x > 0. {
                            -camera.offset.x
                        } else {
                            camera.offset.x
                        };
                }
                if diff_abs.y > camera.offset.y {
                    camera_transform.translation.y = target_transform.translation.y
                        - if diff.y > 0. {
                            -camera.offset.y
                        } else {
                            camera.offset.y
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
        app.add_systems(Update, debug);
    }
}

fn debug(
    mut gizmos: Gizmos,
    mut config: ResMut<GizmoConfig>,
    query_cameras: Query<(&Transform, &Camera)>,
) {
    config.line_width = 1.0;

    for (camera_transform, camera) in &query_cameras {
        gizmos.rect_2d(
            camera_transform.translation.truncate(),
            0.,
            camera.offset * 2.0,
            Color::RED,
        );
    }
}
