use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayerShipPlugin;

impl Plugin for PlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ship).add_systems(
            Update,
            (
                update_player_position,
                handle_player_intersections_with_wall,
            ),
        );
    }
}

pub struct PlayerWallTransportPlugin;
impl Plugin for PlayerWallTransportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}
