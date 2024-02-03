use crate::damage::Damageable;
use crate::meteors::components::Meteor;
use crate::planets::components::Planet;
use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use std::f32::consts::PI;

use crate::damage::lib as damage_lib;
use crate::shots::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::world;
use crate::world::components::{BottomWall, LeftWall, RightWall, TopWall};
use crate::world::resources::WorldCoordinates;
use crate::world::systems as world_systems;

use super::components::*;

pub const PLAYER_SHIP: &str = "playerShip2_orange.png";
pub const PLAYER_ROTATION_SPEED: f32 = 7.0;
pub const PLAYER_ACCELERATION: f32 = 35.0;
pub const PLAYER_SHIP_DENSITY: f32 = 0.9;
pub const PLAYER_SHIP_SCALE: f32 = 0.4;

pub const PLAYER_HEALTH: f32 = 2000.;

pub fn spawn_ship(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
) {
    let window = window_query.get_single().unwrap();
    world_systems::spawn_sprite_frame_at_position(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &sprite_loader,
        PLAYER_SHIP,
        1,
        1,
        0,
        PLAYER_SHIP_SCALE,
        PlayerShip {
            density: PLAYER_SHIP_DENSITY,
            health: PLAYER_HEALTH,
        },
        world::RigidBodyBehaviors::default()
            .with_velocity(LinearVelocity::ZERO)
            .with_external_force(ExternalForce::default())
            .with_density(PLAYER_SHIP_DENSITY),
        Transform::from_xyz(window.width() / 3., window.height() / 3., 0.0),
        Some(WeaponFireTimer { ..default() }),
    );
}

pub fn despawn_player(mut commands: Commands, player_ship_query: Query<Entity, With<PlayerShip>>) {
    if let Ok(player_entity) = player_ship_query.get_single() {
        _despawn(&mut commands, player_entity);
    }
}

fn _despawn(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn();
}

pub fn update_player_position(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_ship_query: Query<
        (&Transform, &mut LinearVelocity, &mut ExternalForce),
        With<PlayerShip>,
    >,
) {
    if let Ok((transform, mut velocity, mut forces)) = player_ship_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let force = transform.rotation.mul_vec3(Vec3::Y) * PLAYER_ACCELERATION;
            forces.apply_force(Vec2::new(force.x, force.y));
        }

        if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_released(KeyCode::W) {
            forces.clear();
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            let force = transform.rotation.mul_vec3(Vec3::Y) * -PLAYER_ACCELERATION;
            forces.apply_force(Vec2::new(force.x, force.y));
        }

        if keyboard_input.just_released(KeyCode::Down) || keyboard_input.just_released(KeyCode::S) {
            forces.clear();
        }
    }
}

pub fn update_player_position_from_coordinates(
    coordinates: ResMut<WorldCoordinates>,
    mut player_ship_query: Query<&mut Transform, With<PlayerShip>>,
) {
    if let Ok(mut transform) = player_ship_query.get_single_mut() {
        let direction = coordinates.0 - Vec2::new(transform.translation.x, transform.translation.y);
        let angle = direction.y.atan2(direction.x);
        transform.rotation = Quat::from_rotation_z(angle - PI / 2.);
    }
}

pub fn handle_player_intersections_with_wall(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    player_ship_query: Query<
        (Entity, &Transform, &LinearVelocity, &CollidingEntities),
        With<PlayerShip>,
    >,
    left_wall_query: Query<&LeftWall>,
    right_wall_query: Query<&RightWall>,
    top_wall_query: Query<&TopWall>,
    bottom_wall_query: Query<&BottomWall>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok((player_ship_entity, player_ship_transform, velocity, colliding_entities)) =
        player_ship_query.get_single()
    {
        let mut should_spawn_ship = false;
        let mut transform = player_ship_transform.clone();
        let sprite = sprite_loader.get_sprite(PLAYER_SHIP).unwrap();
        let radius = sprite.half_width();
        for other_entity in colliding_entities.iter() {
            if let Ok(_) = left_wall_query.get(*other_entity) {
                let distance = player_ship_transform.translation.x;
                if distance < radius && player_ship_transform.translation.x < 0.0 {
                    should_spawn_ship = true;
                    transform.translation.x = window.width() - radius;
                }
            } else if let Ok(_) = right_wall_query.get(*other_entity) {
                let distance = window.width() - player_ship_transform.translation.x;
                if distance < radius && player_ship_transform.translation.x > window.width() {
                    should_spawn_ship = true;
                    transform.translation.x = radius;
                }
            } else if let Ok(_) = top_wall_query.get(*other_entity) {
                let distance = window.height() - player_ship_transform.translation.y;
                if distance < radius && player_ship_transform.translation.y > window.height() {
                    should_spawn_ship = true;
                    transform.translation.y = radius;
                }
            } else if let Ok(_) = bottom_wall_query.get(*other_entity) {
                let distance = player_ship_transform.translation.y;
                if distance < radius && player_ship_transform.translation.y < 0.0 {
                    should_spawn_ship = true;
                    transform.translation.y = window.height() - radius;
                }
            }
        }

        if should_spawn_ship {
            _despawn(&mut commands, player_ship_entity);
            world_systems::spawn_sprite_frame_at_position(
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                &sprite_loader,
                PLAYER_SHIP,
                1,
                1,
                0,
                PLAYER_SHIP_SCALE,
                PlayerShip {
                    density: PLAYER_SHIP_DENSITY,
                    health: PLAYER_HEALTH,
                },
                world::RigidBodyBehaviors::default()
                    .with_velocity(velocity.clone())
                    .with_density(PLAYER_SHIP_DENSITY),
                transform,
                Some(WeaponFireTimer { ..default() }),
            );
            return;
        }
    }
}

pub fn handle_player_collision_with_meteor(
    mut commands: Commands,
    mut player_ship_query: Query<(Entity, &mut PlayerShip, &CollidingEntities)>,
    meteor_query: Query<&Meteor>,
) {
    damage_lib::handle_collision_with_damageable(
        &mut commands,
        &meteor_query,
        &mut player_ship_query,
    );
}
pub fn handle_player_collision_with_planet(
    mut commands: Commands,
    mut player_ship_query: Query<(Entity, &mut PlayerShip, &CollidingEntities)>,
    planet_query: Query<&Planet>,
) {
    damage_lib::handle_collision_with_damageable(
        &mut commands,
        &planet_query,
        &mut player_ship_query,
    );
}
