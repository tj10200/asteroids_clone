use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_2d::prelude::*;
use rand::{random, Rng};
use std::f32::consts::PI;

use super::components::*;
use super::*;
use crate::game::damage::Damageable;
use crate::game::shots::components::Weapon;
use crate::game::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::game::world;
use crate::game::world::components::{BottomWall, LeftWall, RightWall, TopWall};
use crate::game::world::systems as world_systems;

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
    let mut fragments = Vec::with_capacity(num_fragments);
    let start_positions = equidistant_points_on_circle(origin, explosion_radius, num_fragments);
    let angle_increment = 2.0 * PI / num_fragments as f32;

    for i in 0..num_fragments {
        let speed = rng.gen_range(0.5 * max_speed..max_speed);

        // Calculate the velocity of the fragment
        let angle = angle_increment * i as f32;
        let velocity_x = speed * angle.cos();
        let velocity_y = speed * angle.sin();
        fragments.push((start_positions[i], Vec2::new(velocity_x, velocity_y)));
    }

    fragments
}

fn equidistant_points_on_circle(origin: Vec2, radius: f32, num_points: usize) -> Vec<Vec2> {
    let angle_increment = 2.0 * PI / num_points as f32;

    let mut points = Vec::with_capacity(num_points);
    let mut rng = rand::thread_rng();

    for i in 0..num_points {
        let radius = rng.gen_range(0.75 * radius..=1.5 * radius);
        let angle = angle_increment * i as f32;
        points.push(Vec2::new(
            origin.x + radius * angle.cos(),
            origin.y + radius * angle.sin(),
        ));
    }

    points
}

fn nudge_onto_screen(
    mut fragments: Vec<(Vec2, Vec2)>,
    max_width: f32,
    max_height: f32,
) -> Vec<(Vec2, Vec2)> {
    for fragment in fragments.iter_mut() {
        if fragment.0.x < 0. {
            fragment.0.x = 1.;
        } else if fragment.0.x >= max_width {
            fragment.0.x = max_width - 2.;
        }
        if fragment.0.y < 0. {
            fragment.0.y = 1.;
        } else if fragment.0.y >= max_height {
            fragment.0.y = max_height - 2.;
        }
    }
    fragments
}

fn create_new_meteors_after_destruction(
    meteor: &Meteor,
    transform: &Transform,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    max_x: f32,
    max_y: f32,
) -> Vec<(Meteor, Vec2)> {
    let meteor_sprite = sprite_loader.get_sprite(&meteor.sprite_name).unwrap();
    let mut breakup_meteors = meteor.spawn_next_size();
    let mut res = vec![];
    let fragments = explode_meteor(
        Vec2::new(transform.translation.x, transform.translation.y),
        breakup_meteors.len(),
        meteor_sprite.width * transform.scale.x,
        METEOR_SPEED_RANGE.1,
    );
    let fragments = nudge_onto_screen(fragments, max_x, max_y);
    for (i, (position, velocity)) in fragments.iter().enumerate() {
        breakup_meteors[i].velocity = LinearVelocity(*velocity);
        res.push((breakup_meteors[i].to_owned(), *position))
    }
    res
}

pub fn despawn_meteor(mut commands: Commands, mut meteor_query: Query<Entity, With<Meteor>>) {
    for entity in meteor_query.iter_mut() {
        _despawn(&mut commands, entity);
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
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    shot_query: Query<(Entity, &Weapon, &RayHits)>,
    mut meteor_query: Query<(&mut Meteor, &Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (shot_entity, shot, hits) in shot_query.iter() {
        if let Some(hit) = hits.iter().find(|&&hit| hit.time_of_impact <= 0.1) {
            if let Ok((mut meteor, transform)) = meteor_query.get_mut(hit.entity) {
                meteor.damage(shot);
                if meteor.is_dead() {
                    commands.entity(hit.entity).despawn();
                    for new_meteors in create_new_meteors_after_destruction(
                        &meteor,
                        transform,
                        &sprite_loader,
                        window.width(),
                        window.height(),
                    )
                    .iter()
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
            commands.entity(shot_entity).despawn();
            break;
        }
    }
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
