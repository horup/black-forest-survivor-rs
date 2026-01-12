use std::collections::HashMap;
use crate::{CollisionEvent, Frame, TickEvent, event::Event};
use super::Ctx;

/// handles movement of entities in the world
/// also handled collision resolution
pub fn movement_system(tick_event: &TickEvent, ctx: &mut dyn Ctx) {
    let dt = tick_event.dt;
    let world = ctx.world_mut();
    let mut entities = Vec::new();
    world.entities(&mut entities);
    let mut close_entities = Vec::new();
    let mut colliding_entities = HashMap::new();
    for entity_id in entities {
        close_entities.clear();
        colliding_entities.clear();
        let Some(entity) = world.entity(entity_id) else {
            continue;
        };
        let entity_vel = entity.move_dir * dt * entity.max_speed;
        let entity_pos = entity.pos;
        let entity_solid = entity.solid;
        let entity_radius = entity.radius;
        let entity_tile_index = entity.tile_index();

        if entity_vel.length() == 0.0 {
            if let Some(entity_mut) = world.entities.get_mut(entity_id) {
                if entity_mut.is_ability_in_progress() == false {
                    if entity_mut.health.is_alive() {
                        entity_mut.frame = Frame::Default;
                    }
                }
                if entity_mut.move_sinus != 0.0 {
                    entity_mut.move_sinus /= 2.0;

                    if entity_mut.move_sinus.abs() < 0.01 {
                        entity_mut.move_sinus = 0.0;
                        entity_mut.move_distance_total = 0.0;
                    }
                }
            }
            continue;
        }

        // first remove entity from current tile
        if let Some(tile) = world.tiles.get_mut(entity_tile_index) {
            tile.entities.remove(&entity_id);
        }

        // now move the entity
        let mut entity_pos = entity_pos + entity_vel;

        // check and resolve collision with solid entities
        if entity_solid {
            world.get_entities(entity_pos.truncate().as_ivec2(), 2.0, &mut close_entities);
        }

        for other_entity_id in &close_entities {
            if *other_entity_id == entity_id {
                continue;
            }
            let Some(other_entity) = world.entity(*other_entity_id) else {
                continue;
            };
            if !other_entity.solid {
                continue;
            }

            let to_other = other_entity.pos - entity_pos;
            let dist = to_other.length();
            let min_dist = entity_radius + other_entity.radius;
         
            if dist < min_dist && dist > 0.0 {
                let overlap = min_dist - dist;
                let correction = to_other.normalize() * overlap;
                entity_pos += -correction;
                colliding_entities.insert(*other_entity_id, ());
            }
        }

        // finally update entity position
        if let Some(entity_mut) = world.entities.get_mut(entity_id) {
            let old_pos = entity_mut.pos;
            let moved_distance = (entity_pos - old_pos).length();
            entity_mut.move_distance_total += moved_distance * entity_mut.move_sinus_speed;
            entity_mut.pos = entity_pos;
            let move_sinus = entity_mut.move_sinus;
            entity_mut.move_sinus = entity_mut.move_distance_total.sin();
            if move_sinus.signum() != entity_mut.move_sinus.signum() {
                // TODO seperate from this system
                if entity_mut.is_ability_in_progress() == false {
                    if move_sinus < 0.0 {
                        entity_mut.frame = Frame::Walk1;
                    } else {
                        entity_mut.frame = Frame::Walk2;
                    }
                }
            }
        }

        // add entity to new tile
        let new_tile_index = entity_pos.truncate().as_ivec2();
        if let Some(tile) = world.tiles.get_mut(new_tile_index) {
            tile.entities.insert(entity_id, ());
        }

        for other_entity_id in colliding_entities.keys() {
            world.events.push_back(Event::Collision(CollisionEvent {
                entity_1_id: entity_id.clone(),
                entity_2_id: other_entity_id.clone(),
            }));
        }
    }
}
