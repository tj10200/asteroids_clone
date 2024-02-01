use crate::damage::{Damage, Damageable};
use bevy::prelude::*;
use bevy::prelude::{Commands, Entity, Query};
use bevy_xpbd_2d::prelude::*;

pub fn handle_collision_with_damageable<T: Damage + Component, Q: Damageable + Component>(
    commands: &mut Commands,
    damage_query: &Query<&T>,
    damageable_query: &mut Query<(Entity, &mut Q, &CollidingEntities)>,
) {
    if let Ok((damageable_entity, mut damageable, colliding_entities)) =
        damageable_query.get_single_mut()
    {
        for other_entity in colliding_entities.iter() {
            if let Ok(damage_inst) = damage_query.get(*other_entity) {
                damageable.damage(damage_inst);
                if damageable.is_dead() {
                    commands.entity(damageable_entity).despawn();
                }
            }
        }
    }
}
