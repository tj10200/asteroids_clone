use crate::meteors::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::world;
use crate::world::components::{BottomWall, LeftWall, RightWall, TopWall};
use crate::world::systems as world_systems;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::process::Command;

use crate::meteors::components::*;
use crate::shots::components::Weapon;
use rand::random;

pub const NUMBER_OF_METEORS: u32 = 3;

pub fn spawn_meteors(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
) {
    let window = window_query.get_single().unwrap();

    for i in 0..=NUMBER_OF_METEORS {
        let meteor = Meteor::default();

        spawn_meteor_helper(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            &sprite_loader,
            &window,
            meteor,
        )
    }
}

fn spawn_meteor_helper(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    window: &Window,
    meteor: Meteor,
) {
    let sprite_name = &meteor.sprite_name.clone();
    let density = meteor.density;
    let mut velocity = meteor.velocity.clone();
    velocity.angvel = meteor.rotation;
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    world_systems::spawn_sprite_frame_at_position(
        commands,
        asset_server,
        texture_atlases,
        sprite_loader,
        sprite_name,
        meteor.frame_cols,
        meteor.frame_rows,
        meteor.start_frame,
        1.0,
        meteor,
        world::RigidBodyBehaviors::default()
            .with_velocity(velocity)
            .with_density(density),
        Transform::from_xyz(random_x, random_y, 0.0),
        None::<SpriteBundle>,
    );
}

pub fn despawn_meteor(mut commands: Commands, mut meteor_query: Query<(Entity, &Meteor)>) {
    for (entity, meteor) in meteor_query.iter_mut() {
        if meteor.destroyed() {
            _despawn(&mut commands, entity);
        }
    }
}

fn _despawn(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).despawn();
}

pub fn handle_meteor_intersections_with_wall(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    rapier_context: Res<RapierContext>,
    meteor_query: Query<(Entity, &Transform, &Meteor)>,
    left_wall_query: Query<Entity, With<LeftWall>>,
    right_wall_query: Query<Entity, With<RightWall>>,
    top_wall_query: Query<Entity, With<TopWall>>,
    bottom_wall_query: Query<Entity, With<BottomWall>>,
) {
    let window = window_query.get_single().unwrap();
    for (entity, transform, meteor) in meteor_query.iter() {
        // let meteor = &Meteor::default();
        let mut should_spawn = false;
        let mut transform = transform.clone();
        let sprite = sprite_loader.get_sprite(&meteor.sprite_name).unwrap();
        let radius = sprite.half_width();
        if let Ok(wall_entity) = left_wall_query.get_single() {
            if rapier_context.intersection_pair(entity, wall_entity) == Some(true) {
                let distance = transform.translation.x;
                if distance < radius && transform.translation.x < 0.0 {
                    should_spawn = true;
                    transform.translation.x = window.width() - radius;
                }
            }
        }
        if let Ok(wall_entity) = right_wall_query.get_single() {
            if rapier_context.intersection_pair(entity, wall_entity) == Some(true) {
                let distance = window.width() - transform.translation.x;
                if distance < radius && transform.translation.x > window.width() {
                    should_spawn = true;
                    transform.translation.x = radius;
                }
            }
        }
        if let Ok(wall_entity) = top_wall_query.get_single() {
            if rapier_context.intersection_pair(entity, wall_entity) == Some(true) {
                let distance = window.height() - transform.translation.y;
                if distance < radius && transform.translation.y > window.height() {
                    should_spawn = true;
                    transform.translation.y = radius;
                }
            }
        }
        if let Ok(wall_entity) = bottom_wall_query.get_single() {
            if rapier_context.intersection_pair(entity, wall_entity) == Some(true) {
                let distance = transform.translation.y;
                if distance < radius && transform.translation.y < 0.0 {
                    should_spawn = true;
                    transform.translation.y = window.height() - radius;
                }
            }
        }

        if should_spawn {
            let meteor = (*meteor).clone();
            let density = meteor.density;
            let sprite_name = meteor.sprite_name.clone();
            let velocity = meteor.velocity.clone();
            _despawn(&mut commands, entity);
            world_systems::spawn_sprite_frame_at_position(
                &mut commands,
                &asset_server,
                &mut texture_atlases,
                &sprite_loader,
                &sprite_name,
                meteor.frame_cols,
                meteor.frame_rows,
                meteor.start_frame,
                1.0,
                meteor,
                world::RigidBodyBehaviors::default()
                    .with_velocity(velocity)
                    .with_density(density),
                transform,
                None::<SpriteBundle>,
            );
        }
    }
}

pub fn handle_weapon_collision(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut meteor_query: Query<(Entity, &mut Meteor)>,
    shot_query: Query<(Entity, &Weapon)>,
) {
    for (meteor_entity, mut meteor) in meteor_query.iter_mut() {
        for (shot_entity, weapon) in shot_query.iter() {
            if let Some(contact_pair) = rapier_context.contact_pair(meteor_entity, shot_entity) {
                if contact_pair.has_any_active_contacts() {
                    commands.entity(shot_entity).despawn();
                    meteor.damage(weapon);
                }
            }
        }
    }
}
