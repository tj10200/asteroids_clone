use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

mod meteors;
mod planets;
pub mod player;
mod shots;
mod sprite_loader;
pub mod util;
pub mod world;

use meteors::*;
use planets::*;
use player::*;
use shots::WeaponFirePlugin;
use sprite_loader::mapper::XMLSpriteSheetLoader;
use world::*;

pub const PIXELS_PER_METER: f32 = 100.0;
pub const MAIN_SPRITE_SHEET: &str = "sprites/sheet.png";
pub const MAIN_SPRITE_SHEET_MAPPING: &str = "sprites/sheet.xml";
pub const MAIN_SPRITE_SHEET_EDGE_SHAPES: &str = "sprites/sheet1-edges.json";

fn main() {
    App::new()
        .insert_resource(
            XMLSpriteSheetLoader::build(
                MAIN_SPRITE_SHEET,
                MAIN_SPRITE_SHEET_MAPPING,
                MAIN_SPRITE_SHEET_EDGE_SHAPES,
            )
            .unwrap(),
        )
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            scaled_shape_subdivision: 100,
            ..default()
        })
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldPlugin {})
        .add_plugins(PlayerShipPlugin {})
        .add_plugins(WeaponFirePlugin {})
        .add_plugins(MeteorPlugin {})
        .add_plugins(PlanetsPlugin {})
        .run();
}
