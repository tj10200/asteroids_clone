use bevy::prelude::*;

// Defines what state the running application is in
#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
