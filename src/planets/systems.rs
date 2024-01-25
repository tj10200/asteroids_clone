use super::{MAIN_PLANET_DENSITY, MAIN_PLANET_RADIUS};
use crate::meteors::components::Meteor;
use crate::player::components::PlayerShip;
use bevy::asset::TrackAssets;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::dynamics::ExternalForce;
use bevy_rapier2d::prelude::{ColliderMassProperties, ExternalImpulse, Velocity};

use super::components::*;

pub fn spawn_planets(
    mut commands: Commands,
    mut gizmos: Gizmos,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let planet = Planet::new(
        Vec2::new(window.width() / 2., window.height() / 2.),
        MAIN_PLANET_RADIUS,
        MAIN_PLANET_DENSITY,
    );
    gizmos.circle_2d(planet.coordinates.clone(), planet.radius, Color::SILVER);
    commands.spawn(planet);
}

pub fn simulate_meteor_gravity_toward_planets(
    mut commands: Commands,
    planet_query: Query<&Planet>,
    mut meteor_query: Query<(Entity, &Transform, &mut Velocity, &Meteor)>,
) {
    for planet in planet_query.iter() {
        for (entity, transform, mut velocity, meteor) in meteor_query.iter_mut() {
            commands.entity(entity).try_insert(ExternalImpulse {
                impulse: gravitational_velocity(
                    Vec2::from((transform.translation.x, transform.translation.y)),
                    Vec2::from((planet.coordinates.x, planet.coordinates.y)),
                    planet.gravity(meteor.density),
                ),
                torque_impulse: 0.,
            });
        }
    }
}

pub fn simulate_player_gravity_toward_planets(
    mut commands: Commands,
    planet_query: Query<&Planet>,
    mut player_query: Query<(
        Entity,
        &Transform,
        &Velocity,
        &mut ExternalForce,
        &PlayerShip,
    )>,
) {
    for planet in planet_query.iter() {
        for (entity, transform, velocity, mut force, player_ship) in player_query.iter_mut() {
            commands.entity(entity).try_insert(ExternalImpulse {
                impulse: gravitational_velocity(
                    Vec2::from((transform.translation.x, transform.translation.y)),
                    Vec2::from((planet.coordinates.x, planet.coordinates.y)),
                    planet.gravity(player_ship.density),
                ),
                torque_impulse: 0.,
            });
        }
    }
}

fn gravitational_velocity(
    moving_object_pos: Vec2,
    stationary_object_pos: Vec2,
    gravity_strength: f32,
) -> Vec2 {
    let direction = stationary_object_pos - moving_object_pos;
    let distance = direction.length();

    // Normalize the direction vector and scale by gravity strength
    // The strength can be adjusted to simulate stronger or weaker gravity
    if distance != 0.0 {
        direction.normalize() * (gravity_strength / distance)
    } else {
        Vec2::ZERO
    }
}
