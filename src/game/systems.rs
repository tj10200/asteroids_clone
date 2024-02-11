use crate::game::states::SimulationState;
use bevy::prelude::*;

pub fn pause_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
    next_sim_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut next_sim_state: ResMut<NextState<SimulationState>>) {
    next_sim_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match *simulation_state.get() {
            SimulationState::Running => {
                next_sim_state.set(SimulationState::Paused);
                println!("Simulation paused!");
            }
            SimulationState::Paused => {
                next_sim_state.set(SimulationState::Running);
                println!("Simulation running!");
            }
        };
    }
}
