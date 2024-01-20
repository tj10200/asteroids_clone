use bevy::ecs::system::{EntityCommands, Insert};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::prelude::ColliderMassProperties::Density;
use bevy_rapier2d::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_walls);
    }
}

pub struct RigidBodyBehaviors {
    body_type: RigidBody,
    gravity: f32,
    mass: ColliderMassProperties,
    can_sleep: bool,
    ccd_enabled: bool,
    active_events: ActiveEvents,
    velocity: Option<Velocity>,
    external_force: Option<ExternalForce>,
}

impl RigidBodyBehaviors {
    pub fn default() -> Self {
        RigidBodyBehaviors {
            body_type: RigidBody::Dynamic,
            gravity: 0.0,
            mass: ColliderMassProperties::default(),
            can_sleep: false,
            ccd_enabled: true,
            active_events: ActiveEvents::COLLISION_EVENTS,
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
        self.mass = Density(d);
        self
    }
    pub fn add_to_entity(&self, entity: Entity, commands: &mut Commands) {
        commands.add(Insert {
            entity,
            bundle: (
                self.body_type,
                GravityScale(self.gravity),
                if self.can_sleep {
                    Sleeping::default()
                } else {
                    Sleeping::disabled()
                },
                if self.ccd_enabled {
                    Ccd::enabled()
                } else {
                    Ccd::disabled()
                },
                self.active_events,
                self.mass,
            ),
        });

        if let Some(velocity) = self.velocity {
            commands.add(Insert {
                entity,
                bundle: velocity,
            });
        }
        if let Some(external_force) = self.external_force {
            commands.add(Insert {
                entity,
                bundle: external_force,
            });
        }
    }
}

// );
