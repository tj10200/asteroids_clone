use bevy::prelude::*;

// Defines when the game is running or stopped
#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
