use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodyType;
use bevy_rapier2d::rapier::prelude::RigidBodyBuilder;
use std::f32::consts::PI;

use crate::shots::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::util::angle_between;
use crate::world;
use crate::world::components::{BottomWall, LeftWall, RightWall, TopWall};
use crate::world::resources::WorldCoordinates;
use crate::world::systems as world_systems;

use super::components::*;

pub const PLAYER_SHIP: &str = "playerShip2_orange.png";
pub const PLAYER_ROTATION_SPEED: f32 = 7.0;
pub const PLAYER_ACCELERATION: f32 = 35.0;
pub const PLAYER_SHIP_DENSITY: f32 = 1.;
pub const PLAYER_SHIP_SCALE: f32 = 0.4;

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
        },
        world::RigidBodyBehaviors::default()
            .with_velocity(Velocity::zero())
            .with_external_force(ExternalForce::default())
            .with_density(PLAYER_SHIP_DENSITY),
        Transform::from_xyz(window.width() / 3., window.height() / 3., 0.0),
        Some((WeaponFireTimer { ..default() })),
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
    mut player_ship_query: Query<(&Transform, &mut Velocity, &mut ExternalForce), With<PlayerShip>>,
) {
    if let Ok((transform, mut velocity, mut forces)) = player_ship_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let rotation = transform.rotation.to_scaled_axis();
            let linvel = Vec2::from_angle(rotation.z).rotate(Vec2::Y) * PLAYER_ACCELERATION;
            forces.force = linvel;
        }

        if keyboard_input.just_released(KeyCode::Up) || keyboard_input.just_released(KeyCode::W) {
            forces.force = Vec2::ZERO;
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            let rotation = transform.rotation.to_scaled_axis();
            let linvel = Vec2::from_angle(rotation.z).rotate(Vec2::Y) * -PLAYER_ACCELERATION;
            forces.force = linvel;
        }

        if keyboard_input.just_released(KeyCode::Down) || keyboard_input.just_released(KeyCode::S) {
            forces.force = Vec2::ZERO;
        }

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            velocity.angvel = PLAYER_ROTATION_SPEED;
        }

        if keyboard_input.just_released(KeyCode::Left) || keyboard_input.just_released(KeyCode::A) {
            velocity.angvel = 0.0;
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            velocity.angvel = -PLAYER_ROTATION_SPEED;
        }

        if keyboard_input.just_released(KeyCode::Right) || keyboard_input.just_released(KeyCode::D)
        {
            velocity.angvel = 0.0;
        }
    }
}

pub fn update_player_position_from_coordinates(
    coordinates: ResMut<WorldCoordinates>,
    mut player_ship_query: Query<(&mut Transform, &mut Velocity), With<PlayerShip>>,
) {
    if let Ok((mut transform, mut velocity)) = player_ship_query.get_single_mut() {
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
    rapier_context: Res<RapierContext>,
    player_ship_query: Query<(Entity, &Transform, &Velocity, &ExternalForce), With<PlayerShip>>,
    left_wall_query: Query<Entity, With<LeftWall>>,
    right_wall_query: Query<Entity, With<RightWall>>,
    top_wall_query: Query<Entity, With<TopWall>>,
    bottom_wall_query: Query<Entity, With<BottomWall>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok((player_ship_entity, player_ship_transform, velocity, external_force)) =
        player_ship_query.get_single()
    {
        let mut should_spawn_ship = false;
        let mut transform = player_ship_transform.clone();
        let sprite = sprite_loader.get_sprite(PLAYER_SHIP).unwrap();
        let radius = sprite.half_width();
        if let Ok(wall_entity) = left_wall_query.get_single() {
            if rapier_context.intersection_pair(player_ship_entity, wall_entity) == Some(true) {
                let distance = player_ship_transform.translation.x;
                if distance < radius && player_ship_transform.translation.x < 0.0 {
                    should_spawn_ship = true;
                    transform.translation.x = window.width() - radius;
                }
            }
        }
        if let Ok(wall_entity) = right_wall_query.get_single() {
            if rapier_context.intersection_pair(player_ship_entity, wall_entity) == Some(true) {
                let distance = window.width() - player_ship_transform.translation.x;
                if distance < radius && player_ship_transform.translation.x > window.width() {
                    should_spawn_ship = true;
                    transform.translation.x = radius;
                }
            }
        }
        if let Ok(wall_entity) = top_wall_query.get_single() {
            if rapier_context.intersection_pair(player_ship_entity, wall_entity) == Some(true) {
                let distance = window.height() - player_ship_transform.translation.y;
                if distance < radius && player_ship_transform.translation.y > window.height() {
                    should_spawn_ship = true;
                    transform.translation.y = radius;
                }
            }
        }
        if let Ok(wall_entity) = bottom_wall_query.get_single() {
            if rapier_context.intersection_pair(player_ship_entity, wall_entity) == Some(true) {
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
                },
                world::RigidBodyBehaviors::default()
                    .with_velocity(velocity.clone())
                    .with_external_force(external_force.clone())
                    .with_density(PLAYER_SHIP_DENSITY),
                transform,
                Some((WeaponFireTimer { ..default() })),
            );
        }
    }
}
