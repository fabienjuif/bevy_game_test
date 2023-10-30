use bevy::prelude::{Component, Vec3};

pub const GAME_MAX_WIDTH: f32 = 2000.;
pub const GAME_MAX_HEIGHT: f32 = 2000.;

#[derive(Component)]
pub struct Target {
    pub position: Vec3,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Name(pub String);
