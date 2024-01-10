use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct WeaponFirePlugin;

impl Plugin for WeaponFirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_fire_weapon, handle_shot_intersections_with_wall),
        );
    }
}
