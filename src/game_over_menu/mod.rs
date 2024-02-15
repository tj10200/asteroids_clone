use bevy::prelude::*;

mod components;
mod styles;
mod systems;

use crate::states::AppState;
use systems::interactions::*;
use systems::layout::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(
                Update,
                (interact_with_restart_button, interact_with_quit_button)
                    .run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu);
    }
}
