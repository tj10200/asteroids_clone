use bevy::ecs::system::Insert;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::{random, thread_rng};

use crate::components::MainCamera;
use crate::game::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::game::world::RigidBodyBehaviors;

use super::components::*;
use super::resources::*;

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
        .insert(RigidBody::Static)
        .insert(Sensor)
        .insert(Collider::cuboid(window.width(), 0.5));

    // Left Wall
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, half_window.1, 0.0),
                ..default()
            },
            LeftWall {},
        ))
        .insert(RigidBody::Static)
        .insert(Sensor)
        .insert(Collider::cuboid(0.5, window.height()));

    // Top Wall
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(half_window.0, window.height() - 1.0, 0.0),
                ..default()
            },
            TopWall {},
        ))
        .insert(RigidBody::Static)
        .insert(Sensor)
        .insert(Collider::cuboid(window.width(), 0.5));

    // Right Wall
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(window.width() - 1.0, half_window.1, 0.0),
                ..default()
            },
            RightWall {},
        ))
        .insert(RigidBody::Static)
        .insert(Sensor)
        .insert(Collider::cuboid(0.5, window.height()));
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
    let collider = sprite_loader
        .get_sprite_collider(sprite_name, start_frame, true)
        .unwrap();
    spawn_sprite_frame_at_position_with_collider(
        commands,
        asset_server,
        texture_atlases,
        sprite_loader,
        sprite_name,
        frame_cols,
        frame_rows,
        start_frame,
        scale,
        component,
        physics_bundle,
        transform,
        extras,
        collider,
    )
}

pub fn spawn_sprite_frame_at_position_with_collider<T: Component, B: Bundle>(
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
    collider: Collider,
) {
    let texture_handle = asset_server.load(&sprite_loader.file);
    let sprite = sprite_loader.get_sprite(sprite_name).unwrap();
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

pub fn handle_mapping_cursor_to_world(
    mut coords: ResMut<WorldCoordinates>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.0 = world_position;
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

pub fn random_val_outside_contraints(window_size: f32, left_bound: f32, right_bound: f32) -> f32 {
    let uniform = if random() {
        Uniform::from(0.01..left_bound)
    } else {
        Uniform::from(right_bound..0.99)
    };
    let mut rng = thread_rng();
    uniform.sample(&mut rng) * window_size
}
