use bevy::ecs::system::Insert;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::{RigidBodyBuilder, RigidBodySet};

pub mod components;
pub mod resources;
pub mod systems;

use crate::world::resources::WorldCoordinates;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldCoordinates {
            0: Default::default(),
        })
        .add_systems(Startup, (spawn_camera, spawn_walls))
        .add_systems(Update, handle_mapping_cursor_to_world);
    }
}

pub struct RigidBodyBehaviors {
    body_type: RigidBody,
    gravity: f32,
    mass: ColliderMassProperties,
    can_sleep: bool,
    ccd_enabled: bool,
    active_events: ActiveEvents,
    translation: Vec2,
    rotation: f32,
    velocity: Option<Velocity>,
    external_force: Option<ExternalForce>,
}

impl RigidBodyBehaviors {
    pub fn default() -> Self {
        RigidBodyBehaviors {
            body_type: RigidBody::Dynamic,
            gravity: 0.0,
            mass: ColliderMassProperties::Density(1.0),
            can_sleep: false,
            ccd_enabled: true,
            active_events: ActiveEvents::COLLISION_EVENTS,
            translation: Vec2::default(),
            rotation: 0.0,
            velocity: None,
            external_force: None,
        }
    }

    pub fn with_velocity(&mut self, v: Velocity) -> &mut Self {
        self.velocity = Some(v);
        self
    }

    pub fn with_external_force(&mut self, f: ExternalForce) -> &mut Self {
        self.external_force = Some(f);
        self
    }

    pub fn with_density(&mut self, d: f32) -> &mut Self {
        self.mass = ColliderMassProperties::Density(d);
        self
    }

    pub fn add_to_entity(&self, entity: Entity, commands: &mut Commands) {
        commands
            .entity(entity)
            .insert(self.body_type)
            .insert(GravityScale(self.gravity))
            .insert(if self.can_sleep {
                Sleeping::disabled()
            } else {
                Sleeping::disabled()
            })
            .insert(if self.ccd_enabled {
                Ccd::enabled()
            } else {
                Ccd::disabled()
            })
            .insert(self.active_events)
            .insert(self.mass);

        // commands.entity(entity).insert((
        //     self.body_type,
        //     GravityScale(self.gravity),
        //     if self.can_sleep {
        //         Sleeping::default()
        //     } else {
        //         Sleeping::disabled()
        //     },
        //     if self.ccd_enabled {
        //         Ccd::enabled()
        //     } else {
        //         Ccd::disabled()
        //     },
        //     self.active_events,
        //     self.mass,
        // ));

        if let Some(velocity) = self.velocity {
            commands.entity(entity).insert(velocity);
        }

        if let Some(external_force) = self.external_force {
            commands.entity(entity).insert(external_force);
        }
    }
}

// );
