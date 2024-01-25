use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

mod meteors;
mod planets;
pub mod player;
mod shots;
mod sprite_loader;
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
        .add_systems(Startup, spawn_camera)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// pub fn spawn_ship(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
// ) {
//     let window = window_query.get_single().unwrap();
//     let texture_handle = asset_server.load("sprites/sheet.png");
//     let ship_width = 99.0;
//     let ship_height = 75.0;
//     let ship_offset = (211.0, 941.0);
//     let texture_atlas = TextureAtlas::from_grid(
//         texture_handle,
//         Vec2::new(ship_width, ship_height),
//         1,
//         1,
//         None,
//         Some(Vec2::new(ship_offset.0, ship_offset.1)),
//     );
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
//     let pos = (window.width() / 2.0, window.height() / 2.0);
//     let scale = 1.0;
//     commands
//         .spawn(SpriteSheetBundle {
//             texture_atlas: texture_atlas_handle,
//             sprite: TextureAtlasSprite::new(0),
//             ..default()
//         })
//         .insert(RigidBody::Dynamic)
//         .insert(Sleeping::disabled())
//         .insert(Ccd::enabled())
//         .insert(Collider::ball(ship_width / 2.0))
//         .insert(Transform::from_xyz(pos.0, pos.1, 0.0).with_scale(Vec3::splat(scale)))
//         .insert(ActiveEvents::COLLISION_EVENTS)
//         .insert(player_components::PlayerShip);
// }
