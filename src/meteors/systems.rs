use crate::meteors::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::world;
use crate::world::components::{BottomWall, LeftWall, RightWall, TopWall};
use crate::world::systems as world_systems;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use std::f32::consts::PI;

use super::*;
use crate::damage::lib as damage_lib;
use crate::damage::Damageable;
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

    for _ in 0..=NUMBER_OF_METEORS {
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

pub fn spawn_meteors_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    meteor_spawn_timer: Res<MeteorSpawnTimer>,
) {
    if meteor_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        spawn_meteor_at_random_location(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            &sprite_loader,
            &window,
            Meteor::default(),
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
    let (width, height) = (window.width(), window.height());
    let (random_x, random_y) = if random() {
        (
            world_systems::random_val_outside_contraints(
                width,
                METEOR_SPAWN_RANGE_REL_TO_WINDOW.0,
                METEOR_SPAWN_RANGE_REL_TO_WINDOW.1,
            ),
            random::<f32>() * height,
        )
    } else {
        (
            random::<f32>() * width,
            world_systems::random_val_outside_contraints(
                height,
                METEOR_SPAWN_RANGE_REL_TO_WINDOW.0,
                METEOR_SPAWN_RANGE_REL_TO_WINDOW.1,
            ),
        )
    };
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
        breakup_meteors[i].velocity = LinearVelocity(*velocity);
        res.push((breakup_meteors[i].to_owned(), *position))
    }
    res
}

pub fn despawn_meteor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    mut meteor_query: Query<(Entity, &Meteor, &Transform)>,
) {
    for (entity, meteor, transform) in meteor_query.iter_mut() {
        if meteor.is_dead() {
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
    meteor_query: Query<(Entity, &Transform, &Meteor, &CollidingEntities)>,
    left_wall_query: Query<Entity, With<LeftWall>>,
    right_wall_query: Query<Entity, With<RightWall>>,
    top_wall_query: Query<Entity, With<TopWall>>,
    bottom_wall_query: Query<Entity, With<BottomWall>>,
) {
    let window = window_query.get_single().unwrap();
    for (entity, transform, meteor, colliding_entities) in meteor_query.iter() {
        // let meteor = &Meteor::default();
        let mut should_spawn = false;
        let mut transform = transform.clone();
        let sprite = sprite_loader.get_sprite(&meteor.sprite_name).unwrap();
        let radius = sprite.half_width();
        for other_entity in colliding_entities.iter() {
            if let Ok(_) = left_wall_query.get(*other_entity) {
                let distance = transform.translation.x;
                if distance < radius && transform.translation.x < 0.0 {
                    should_spawn = true;
                    transform.translation.x = window.width() - radius;
                }
            } else if let Ok(_) = right_wall_query.get(*other_entity) {
                let distance = window.width() - transform.translation.x;
                if distance < radius && transform.translation.x > window.width() {
                    should_spawn = true;
                    transform.translation.x = radius;
                }
            } else if let Ok(_) = top_wall_query.get(*other_entity) {
                let distance = window.height() - transform.translation.y;
                if distance < radius && transform.translation.y > window.height() {
                    should_spawn = true;
                    transform.translation.y = radius;
                }
            } else if let Ok(_) = bottom_wall_query.get(*other_entity) {
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

pub fn handle_weapon_collision(
    mut commands: Commands,
    shot_query: Query<(&Weapon, &CollidingEntities)>,
    mut meteor_query: Query<&mut Meteor>,
) {
    damage_lib::handle_collision_with_damage(&mut commands, &shot_query, &mut meteor_query);
}

pub fn constrain_meteor_velocity(mut meteor_query: Query<&mut LinearVelocity, With<Meteor>>) {
    let (min, max) = METEOR_SPEED_RANGE;
    for mut velocity in meteor_query.iter_mut() {
        velocity.0.x = match velocity.0.x {
            x if x < min => min,
            x if x > max => max,
            x => x,
        };
        velocity.0.y = match velocity.0.y {
            y if y < min => min,
            y if y > max => max,
            y => y,
        };
    }
}

pub fn tick_meteor_spawn_timer(mut timer: ResMut<MeteorSpawnTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}
