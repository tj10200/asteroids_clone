use super::{MAIN_PLANET_DENSITY, MAIN_PLANET_RADIUS};
use crate::meteors::components::Meteor;
use crate::player::components::PlayerShip;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use super::components::*;

pub fn spawn_planets(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let coordinates = Vec2::new(window.width() / 2., window.height() / 2.);
    let radius = MAIN_PLANET_RADIUS;
    let planet = Planet::new(
        coordinates.clone(),
        radius,
        MAIN_PLANET_DENSITY,
        Color::SEA_GREEN,
    );
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_xyz(coordinates.x, coordinates.y, 0.),
                ..default()
            },
            planet,
        ))
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(radius));
}

pub fn render_planets(mut gizmos: Gizmos, planet_query: Query<&Planet>) {
    for planet in planet_query.iter() {
        let planet = planet.clone();
        gizmos.circle_2d(planet.coordinates, planet.radius, planet.color);
    }
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
