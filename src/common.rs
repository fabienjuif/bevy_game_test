use bevy::prelude::*;

pub const GAME_MAX_WIDTH: f32 = 2000.;
pub const GAME_MAX_HEIGHT: f32 = 2000.;

pub const DEFAULT_HEALTH_COLOR: Color = Color::rgb(0.2, 0.8, 0.2);

#[derive(Component)]
pub struct Target {
    pub position: Vec3,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component, Clone)]
pub struct Team {
    pub id: String,
    pub color: Color,
}

#[derive(Component)]
pub struct Player;
