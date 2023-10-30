use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub health: f32,
}

#[derive(Component)]
pub struct HealthBar {
    pub entity: Entity,
    pub translation: Vec3,
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_health_bar_position);
    }
}

fn update_health_bar_position(
    mut commands: Commands,
    query_health: Query<&GlobalTransform, With<Health>>,
    mut query: Query<(&mut Transform, &HealthBar, Entity)>,
) {
    for (mut transform, health_bar, entity) in &mut query {
        match query_health.get(health_bar.entity) {
            Ok(parent_transform) => {
                transform.translation =
                    parent_transform.to_scale_rotation_translation().2 + health_bar.translation;
            }
            Err(_) => {
                info!("health bar alone, unspawning it");
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
