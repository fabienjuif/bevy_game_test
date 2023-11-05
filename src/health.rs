use bevy::prelude::*;

pub const DEFAULT_HEALTH_COLOR: Color = Color::rgb(0.2, 0.8, 0.2);

#[derive(Component)]
pub struct Health {
    pub value: f32,
    pub max: f32,

    pub add_health_bar: bool,
    pub health_bar_size: Option<Vec2>,
    pub health_bar_position: Option<Vec3>,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            value: max,
            max,
            add_health_bar: true,
            health_bar_position: None,
            health_bar_size: None,
        }
    }

    pub fn with_health_bar_position(mut self, position: Vec3) -> Self {
        self.health_bar_position = Some(position);
        self
    }

    pub fn with_health_bar_size(mut self, size: Vec2) -> Self {
        self.health_bar_size = Some(size);
        self
    }

    pub fn hit(&mut self, value: f32) -> &Self {
        if value < 0. {
            return self;
        }
        self.value -= value;
        if self.value < 0. {
            self.value = 0.;
        }
        self
    }

    pub fn is_dead(&self) -> bool {
        self.value <= 0.
    }
}

#[derive(Component)]
pub struct HealthBar {
    pub entity: Entity,
    pub translation: Vec3,
    pub size: Vec2,
}

#[derive(Bundle)]
struct HealthBarBundle {
    pub sprite: SpriteBundle,
    pub health_bar: HealthBar,
}

impl HealthBarBundle {
    pub fn new(entity: Entity, translation: Vec3, size: Vec2) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: DEFAULT_HEALTH_COLOR,
                    custom_size: Some(size),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            health_bar: HealthBar {
                entity,
                translation,
                size,
            },
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, add_health_bars).add_systems(
            PostUpdate,
            (
                update_health_bar_position,
                update_health_bar_visual,
                clear_orphans_healthbars,
            ),
        );
    }
}

fn clear_orphans_healthbars(
    mut commands: Commands,
    query_health: Query<&Health>,
    mut query: Query<(&HealthBar, Entity)>,
) {
    for (health_bar, entity) in &mut query {
        if !query_health.contains(health_bar.entity) {
            debug!("health bar alone, unspawning it");
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_health_bar_position(
    mut commands: Commands,
    query_health: Query<&GlobalTransform, With<Health>>,
    mut query: Query<(&mut Transform, &HealthBar, Entity)>,
) {
    for (mut transform, health_bar, entity) in &mut query {
        if let Ok(parent_transform) = query_health.get(health_bar.entity) {
            transform.translation =
                parent_transform.to_scale_rotation_translation().2 + health_bar.translation;

            commands.entity(entity).insert(Visibility::Visible);
        }
    }
}

fn update_health_bar_visual(
    query_health: Query<&Health>,
    mut query: Query<(&mut Sprite, &HealthBar)>,
) {
    for (mut sprite, health_bar) in &mut query {
        if let Ok(health) = query_health.get(health_bar.entity) {
            if health.value >= 0. {
                let mut size = health_bar.size;
                size.x = health.value * health_bar.size.x / health.max;
                sprite.custom_size = Some(size);
            }
        }
    }
}

fn add_health_bars(mut commands: Commands, mut query_health: Query<(Entity, &mut Health)>) {
    for (entity, mut health) in &mut query_health {
        if health.add_health_bar {
            health.add_health_bar = false;

            commands.spawn(HealthBarBundle::new(
                entity,
                health
                    .health_bar_position
                    .unwrap_or(Vec3::new(0.0, 15.0, 0.1)),
                health.health_bar_size.unwrap_or(Vec2::new(10.0, 5.0)),
            ));
        }
    }
}
