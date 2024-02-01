use crate::player::components::PlayerShip;
use crate::shots::components::*;
use crate::sprite_loader::mapper::XMLSpriteSheetLoader;
use crate::world;
use crate::world::components::{BottomWall, LeftWall, RightWall, TopWall};
use crate::world::systems as world_systems;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub fn player_fire_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    sprite_loader: Res<XMLSpriteSheetLoader>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut player_ship_query: Query<(&Transform, &mut WeaponFireTimer), With<PlayerShip>>,
    time: Res<Time>,
) {
    if let Ok((transform, mut weapon_fire_timer)) = player_ship_query.get_single_mut() {
        weapon_fire_timer.timer.tick(time.delta());
        let weapon = Weapon::default();
        let sprite_name = weapon.sprite_name.clone();
        if keyboard_input.pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::Space)
            || mouse_input.pressed(MouseButton::Left)
            || mouse_input.just_pressed(MouseButton::Left)
        {
            if weapon_fire_timer.timer.elapsed() >= weapon_fire_timer.fire_delay {
                weapon_fire_timer.timer.reset();
                let rotation = transform.rotation.to_scaled_axis();
                let linvel = Vec2::from_angle(rotation.z).rotate(Vec2::Y) * weapon.speed;
                spawn_weapon_at_position(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    &sprite_loader,
                    &sprite_name,
                    weapon,
                    transform,
                    LinearVelocity(linvel),
                );
            }
        }
    }
}
fn spawn_weapon_at_position(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    sprite_loader: &Res<XMLSpriteSheetLoader>,
    sprite_name: &str,
    weapon: Weapon,
    ship_transform: &Transform,
    force: LinearVelocity,
) {
    let scale = weapon.scale;
    let density = weapon.density;
    world_systems::spawn_sprite_frame_at_position(
        commands,
        asset_server,
        texture_atlases,
        sprite_loader,
        sprite_name,
        weapon.frame_cols,
        weapon.frame_rows,
        weapon.start_frame,
        scale,
        weapon,
        world::RigidBodyBehaviors::default()
            .with_velocity(force.clone())
            .with_density(density),
        middle_shot_from_transform(ship_transform).with_scale(Vec3::splat(scale)),
        Some((WeaponFireTimer { ..default() }, Sensor)),
    );
}

fn middle_shot_from_transform(transform: &Transform) -> Transform {
    let shot_vec = Vec3::new(0.0, FIRE_DISTANCE_FROM_PLAYER, 0.0);
    shot_from_transform(shot_vec, transform)
}

fn shot_from_transform(shot_vec: Vec3, transform: &Transform) -> Transform {
    let angle = transform.rotation.to_scaled_axis().z;
    let angle_cos = angle.cos();
    let angle_sin = angle.sin();
    let new_x = angle_cos * shot_vec.x - angle_sin * shot_vec.y + transform.translation.x;
    let new_y = angle_sin * shot_vec.x + angle_cos * shot_vec.y + transform.translation.y;

    Transform {
        translation: Vec3::new(new_x, new_y, 0.0),
        rotation: transform.rotation.clone(),
        scale: Default::default(),
    }
}

pub fn handle_shot_intersections_with_wall(
    mut commands: Commands,
    shot_query: Query<(Entity, &Transform), With<Weapon>>,
    left_wall_query: Query<&Transform, With<LeftWall>>,
    right_wall_query: Query<&Transform, With<RightWall>>,
    top_wall_query: Query<&Transform, With<TopWall>>,
    bottom_wall_query: Query<&Transform, With<BottomWall>>,
) {
    for (shot_entity, shot_transform) in shot_query.iter() {
        let mut check_and_despawn = |check: bool| {
            if check {
                commands.entity(shot_entity).despawn();
            }
            check
        };
        if let Ok(wall_transform) = left_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.x < wall_transform.translation.x) {
                break;
            }
        }
        if let Ok(wall_transform) = right_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.x > wall_transform.translation.x) {
                break;
            }
        }
        if let Ok(wall_transform) = top_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.y > wall_transform.translation.y) {
                break;
            }
        }
        if let Ok(wall_transform) = bottom_wall_query.get_single() {
            if check_and_despawn(shot_transform.translation.y < wall_transform.translation.y) {
                break;
            }
        }
    }
}
