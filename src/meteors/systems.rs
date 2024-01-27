use crate::meteors::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::world;
use crate::world::components::{BottomWall, LeftWall, RightWall, TopWall};
use crate::world::systems as world_systems;
use bevy::a11y::accesskit::Role::Meter;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

use super::*;
use crate::meteors::components::*;
use crate::shots::components::Weapon;
use rand::{random, Rng};

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

        spawn_meteor_at_random_location(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            &sprite_loader,
            &window,
            meteor,
        )
    }
}

fn spawn_meteor_at_random_location(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    window: &Window,
    meteor: Meteor,
) {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    spawn_meteor_at_position(
        commands,
        asset_server,
        texture_atlases,
        sprite_loader,
        meteor,
        Vec2::new(random_x, random_y),
    )
}

fn spawn_meteor_at_position(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    meteor: Meteor,
    translation: Vec2,
) {
    let sprite_name = &meteor.sprite_name.clone();
    let density = meteor.density;
    let velocity = meteor.velocity.clone();
    world_systems::spawn_sprite_frame_at_position(
        commands,
        asset_server,
        texture_atlases,
        sprite_loader,
        sprite_name,
        meteor.frame_cols,
        meteor.frame_rows,
        meteor.start_frame,
        METEORS_SCALE,
        meteor,
        world::RigidBodyBehaviors::default()
            .with_velocity(velocity)
            .with_density(density),
        Transform::from_xyz(translation.x, translation.y, 0.0),
        None::<SpriteBundle>,
    );
}

// scale can be relative speed or distance from origin
fn rotation_relative_vector(origin: Vec2, rotation_radians: f32, scale: f32) -> Vec2 {
    let dx = rotation_radians.cos() * scale;
    let dy = rotation_radians.sin() * scale;

    Vec2::new(dx, dy)
}

fn explode_meteor(
    origin: Vec2,
    num_fragments: usize,
    explosion_radius: f32,
    max_speed: f32,
) -> Vec<(Vec2, Vec2)> {
    let mut rng = rand::thread_rng();
    let mut fragments = Vec::new();

    for _ in 0..num_fragments {
        let angle = rng.gen_range(0.0..2.0 * PI);
        let distance = rng.gen_range(0.0..explosion_radius);
        let speed = rng.gen_range(0.5 * max_speed..max_speed);

        // Calculate the starting position of the fragment
        let start_x = origin.x + distance * angle.cos();
        let start_y = origin.y + distance * angle.sin();
        let start_position = Vec2::new(start_x, start_y);

        // Calculate the velocity of the fragment
        let velocity_x = speed * angle.cos();
        let velocity_y = speed * angle.sin();
        let velocity = Vec2::new(velocity_x, velocity_y);

        fragments.push((start_position, velocity));
    }

    fragments
}

fn create_new_meteors_after_destruction(
    meteor: &Meteor,
    transform: &Transform,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
) -> Vec<(Meteor, Vec2)> {
    let meteor_sprite = sprite_loader.get_sprite(&meteor.sprite_name).unwrap();
    let mut breakup_meteors = meteor.spawn_next_size();
    let mut res = vec![];
    for (i, (position, velocity)) in explode_meteor(
        Vec2::new(transform.translation.x, transform.translation.y),
        breakup_meteors.len(),
        meteor_sprite.width,
        METEOR_SPEED_RANGE.1,
    )
    .iter()
    .enumerate()
    {
        breakup_meteors[i].velocity = Velocity::linear(*velocity);
        res.push((breakup_meteors[i].to_owned(), *position))
    }
    res
}

pub fn despawn_meteor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    mut meteor_query: Query<(Entity, &Meteor, &Transform)>,
) {
    for (entity, meteor, transform) in meteor_query.iter_mut() {
        if meteor.destroyed() {
            _despawn(&mut commands, entity);

            for new_meteors in
                create_new_meteors_after_destruction(meteor, transform, &sprite_loader).iter()
            {
                spawn_meteor_at_position(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    &sprite_loader,
                    new_meteors.0.clone(),
                    new_meteors.1,
                )
            }
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
                METEORS_SCALE,
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

// pub fn handle_weapon_collision(
//     mut commands: Commands,
//     rapier_context: Res<RapierContext>,
//     mut meteor_query: Query<(Entity, &mut Meteor)>,
//     shot_query: Query<(Entity, &Weapon)>,
// ) {
//     for (meteor_entity, mut meteor) in meteor_query.iter_mut() {
//         for (shot_entity, weapon) in shot_query.iter() {
//             // if let Some(contact_pair) = rapier_context.contact_pair(meteor_entity, shot_entity) {
//             if rapier_context.intersection_pair(meteor_entity, shot_entity) == Some(true) {
//                 commands.entity(shot_entity).despawn();
//                 meteor.damage(weapon);
//             }
//         }
//     }
// }
pub fn handle_weapon_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    shot_query: Query<&Weapon>,
    mut meteor_query: Query<&mut Meteor>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            let (shot_entity, shot, other_entity) = if shot_query.get(*entity1).is_ok() {
                (*entity1, shot_query.get(*entity1).unwrap(), *entity2)
            } else if shot_query.get(*entity2).is_ok() {
                (*entity2, shot_query.get(*entity2).unwrap(), *entity1)
            } else {
                continue;
            };

            if let Ok(mut meteor) = meteor_query.get_mut(other_entity) {
                meteor.damage(shot);
                commands.entity(shot_entity).despawn();
            }
            // Handle the bullet impact, e.g., apply damage, play sound, etc.
            // Remove the bullet from the game
        }
    }
}

pub fn constrain_meteor_velocity(mut meteor_query: Query<&mut Velocity, With<Meteor>>) {
    let (min, max) = METEOR_SPEED_RANGE;
    for mut velocity in meteor_query.iter_mut() {
        velocity.linvel.x = match velocity.linvel.x {
            x if x < min => min,
            x if x > max => max,
            x => x,
        };
        velocity.linvel.y = match velocity.linvel.y {
            y if y < min => min,
            y if y > max => max,
            y => y,
        };
    }
}
