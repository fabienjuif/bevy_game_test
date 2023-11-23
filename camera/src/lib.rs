use bevy::prelude::{
    App, Bundle, Camera2dBundle, Component, Entity, Plugin, PostUpdate, Query, Transform, With,
    Without,
};

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct Camera(pub Entity);

#[derive(Bundle)]
pub struct CameraBundle {
    camera: Camera,
    bundle: Camera2dBundle,
}

impl CameraBundle {
    pub fn new(target: Entity, bundle: Camera2dBundle) -> Self {
        Self {
            camera: Camera(target),
            bundle,
        }
    }

    pub fn new_with_default_bundle(target: Entity) -> Self {
        Self {
            camera: Camera(target),
            bundle: Camera2dBundle::default(),
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, cameraman);
    }
}

fn cameraman(
    mut query_camera: Query<(&mut Transform, &Camera), Without<Target>>,
    query_targets: Query<(&Transform, Entity), With<Target>>,
) {
    for (mut camera_transform, camera) in &mut query_camera {
        for (target_transform, target_entity) in &query_targets {
            if camera.0 != target_entity {
                continue;
            }

            // TODO: for now we follow the first target but we could think of doing an average positions of all the targets
            if camera.0 == target_entity {
                camera_transform.translation.x = target_transform.translation.x;
                camera_transform.translation.y = target_transform.translation.y;

                break;
            }
        }
    }
}
