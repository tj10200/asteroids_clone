use ::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::shots::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;

use super::components::*;

pub const PLAYER_SHIP: &str = "playerShip2_orange.png";
pub const PLAYER_ROTATION_SPEED: f32 = 10.0;
pub const PLAYER_ACCELERATION: f32 = 50.0;
pub fn spawn_ship(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
) {
    let window = window_query.get_single().unwrap();
    spawn_ship_at_position(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &sprite_loader,
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        Velocity::zero(),
        ExternalForce::default(),
    );
}
fn spawn_ship_at_position(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    transform: Transform,
    velocity: Velocity,
    force: ExternalForce,
) {
    let texture_handle = asset_server.load(&sprite_loader.file);
    let sprite = sprite_loader.get_sprite(PLAYER_SHIP.to_string()).unwrap();
    let ship_offset = (sprite.x as f32, sprite.y as f32);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(sprite.width as f32, sprite.height as f32),
        1,
        1,
        None,
        Some(Vec2::new(ship_offset.0, ship_offset.1)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let scale = 1.0;
    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                ..default()
            },
            PlayerShip {},
        ))
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::ball((sprite.width as f32) / 2.0))
        .insert(ColliderMassProperties::Density(0.05))
        .insert(transform.with_scale(Vec3::splat(scale)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(velocity)
        .insert(force)
        .insert(WeaponFireTimer { ..default() });
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
pub fn spawn_walls(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let half_window = (window.width() / 2.0, window.height() / 2.0);

    // Bottom Wall
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(half_window.0, 0.0, 0.0),
                ..default()
            },
            BottomWall {},
        ))
        .insert(RigidBody::Fixed)
        .insert(Sensor)
        .insert(Collider::cuboid(half_window.0, 0.5))
        .insert(ActiveEvents::COLLISION_EVENTS);

    // Left Wall
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, half_window.1, 0.0),
                ..default()
            },
            LeftWall {},
        ))
        .insert(RigidBody::Fixed)
        .insert(Sensor)
        .insert(Collider::cuboid(0.5, half_window.1))
        .insert(ActiveEvents::COLLISION_EVENTS);

    // Top Wall
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(half_window.0, window.height() - 1.0, 0.0),
                ..default()
            },
            TopWall {},
        ))
        .insert(RigidBody::Fixed)
        .insert(Sensor)
        .insert(Collider::cuboid(half_window.0, 0.5))
        .insert(ActiveEvents::COLLISION_EVENTS);

    // Right Wall
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() - 1.0, half_window.1, 0.0),
                ..default()
            },
            RightWall {},
        ))
        .insert(RigidBody::Fixed)
        .insert(Sensor)
        .insert(Collider::cuboid(0.5, half_window.1))
        .insert(ActiveEvents::COLLISION_EVENTS);
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
        let sprite = sprite_loader.get_sprite(PLAYER_SHIP.to_string()).unwrap();
        let radius = sprite.w_radius();
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
            spawn_ship_at_position(
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                &sprite_loader,
                transform,
                velocity.clone(),
                external_force.clone(),
            );
        }
    }
}
