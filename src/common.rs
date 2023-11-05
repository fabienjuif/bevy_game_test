use bevy::prelude::*;

pub const GAME_MAX_WIDTH: f32 = 2000.;
pub const GAME_MAX_HEIGHT: f32 = 2000.;

#[derive(Component)]
pub struct Target {
    pub position: Vec3,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Rewards {
    pub gold: f32,
}
