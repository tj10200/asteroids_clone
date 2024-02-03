use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

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
    mass: Mass,
    velocity: Option<LinearVelocity>,
    external_force: Option<ExternalForce>,
}

impl RigidBodyBehaviors {
    pub fn default() -> Self {
        RigidBodyBehaviors {
            body_type: RigidBody::Dynamic,
            gravity: 0.0,
            mass: Mass(1.0),
            velocity: None,
            external_force: None,
        }
    }

    pub fn with_velocity(&mut self, v: LinearVelocity) -> &mut Self {
        self.velocity = Some(v);
        self
    }

    pub fn with_external_force(&mut self, f: ExternalForce) -> &mut Self {
        self.external_force = Some(f);
        self
    }

    pub fn with_density(&mut self, d: f32) -> &mut Self {
        self.mass = Mass(d);
        self
    }

    pub fn add_to_entity(&self, entity: Entity, commands: &mut Commands) {
        commands
            .entity(entity)
            .insert(self.body_type)
            .insert(GravityScale(self.gravity))
            .insert(ColliderDensity(0.0))
            .insert(self.mass);

        if let Some(velocity) = self.velocity {
            commands.entity(entity).insert(velocity);
        }

        if let Some(external_force) = self.external_force {
            commands.entity(entity).insert(external_force);
        }
    }
}

// );
