use crate::shots::components::WeaponFireTimer;
use ::bevy::prelude::*;
use bevy::ecs::system::Insert;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::ops::Deref;

use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::world::RigidBodyBehaviors;

use super::components::*;

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

pub fn spawn_sprite_frame_at_position<T: Component, B: Bundle>(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    sprite_name: &str,
    frame_cols: usize,
    frame_rows: usize,
    start_frame: usize,
    scale: f32,
    component: T,
    physics_bundle: &RigidBodyBehaviors,
    transform: Transform,
    extras: Option<B>,
) {
    let texture_handle = asset_server.load(&sprite_loader.file);
    let sprite = sprite_loader.get_sprite(sprite_name).unwrap();
    let collider = sprite_loader
        .get_sprite_collider(sprite_name, start_frame, true)
        .unwrap();
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(sprite.width, sprite.height),
        frame_cols,
        frame_rows,
        None,
        Some(Vec2::new(sprite.x, sprite.y)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let spawned = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(start_frame),
                ..default()
            },
            component,
        ))
        .insert(transform.with_scale(Vec3::splat(scale)))
        .insert(collider.clone())
        .id();

    physics_bundle.add_to_entity(spawned, commands);

    if let Some(extras) = extras {
        commands.add(Insert {
            entity: spawned,
            bundle: extras,
        });
    }
}
