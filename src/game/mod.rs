use bevy::app::App;
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};

pub mod damage;
mod meteors;
mod planets;
pub mod player;
mod shots;
pub(crate) mod sprite_loader;
mod states;
mod systems;
pub mod util;
pub mod world;

use super::states::AppState;
use meteors::*;
use planets::*;
use player::*;
use shots::WeaponFirePlugin;
use sprite_loader::mapper::XMLSpriteSheetLoader;
use world::*;

use states::*;
use systems::*;

pub const PIXELS_PER_METER: f32 = 100.0;
pub const MAIN_SPRITE_SHEET: &str = "sprites/sheet.png";
pub const MAIN_SPRITE_SHEET_MAPPING: &str = "sprites/sheet.xml";
pub const MAIN_SPRITE_SHEET_EDGE_SHAPES: &str = "sprites/sheet1-edges.json";
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .insert_resource(
                XMLSpriteSheetLoader::build(
                    MAIN_SPRITE_SHEET,
                    MAIN_SPRITE_SHEET_MAPPING,
                    MAIN_SPRITE_SHEET_EDGE_SHAPES,
                )
                .unwrap(),
            )
            .add_systems(OnEnter(AppState::Game), resume_simulation)
            .add_systems(OnExit(AppState::Game), pause_simulation)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
            // .add_plugins((PhysicsPlugins::default()))
            .add_plugins((WorldPlugin {}))
            .add_plugins(PlayerShipPlugin {})
            .add_plugins(WeaponFirePlugin {})
            .add_plugins(MeteorPlugin {})
            .add_plugins(PlanetsPlugin {});
    }
}
