use bevy::ecs::schedule::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Pause,
    Game,
}
