use bevy::prelude::*;

use crate::states::GameState;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, pause);
    }
}

fn pause(
    current_state: Res<State<GameState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Game => {
                game_state.set(GameState::Pause);
            }
            GameState::Pause => {
                game_state.set(GameState::Game);
            }
            _ => {
                return;
            }
        }
        keyboard_input.reset(KeyCode::Escape);
    }
}
