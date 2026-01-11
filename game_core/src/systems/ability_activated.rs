use glam::Vec2;

use crate::{AbilityActivedEvent, math, event::Event};
use super::Ctx;

pub fn ability_activated_system(event: &AbilityActivedEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    let entity_id = event.entity_id;
    if let Some(e) = world.entities.get_mut(event.entity_id) {
        let facing = e.facing;
        let pos = e.pos;


        let hit_length = 1.0;
        let line1:(Vec2, Vec2) = (
            Vec2::new(pos.x, pos.y),
            Vec2::new(pos.x + facing.cos() * hit_length, pos.y + facing.sin() * hit_length)
        );
       
 
        //let target_pos = pos + Vec3::new(facing.cos(), facing.sin(), 0.0);
        //let mut target_entity_id_opt = None;
        for (other_entity_id, other_entity) in world.entities.iter() {
            if other_entity_id == event.entity_id {
                continue;
            }

            let other_pos = other_entity.pos;
            
            let r = other_entity.radius;
            let line2:(Vec2, Vec2) = (
                Vec2::new(other_pos.x - facing.sin() * r, other_pos.y + facing.cos() * r),
                Vec2::new(other_pos.x + facing.sin() * r, other_pos.y - facing.cos() * r)
            );

            if let Some(_intersection_point) = math::line_intersect(line1, line2) {
                //world.events.push_back(Event::Despawn(DespawnEvent { entity_id: other_entity_id }));
                ctx.push_event(Event::AbilityHit(crate::AbilityHitEvent { entity_id: entity_id, target_entity_id: other_entity_id }));
                break;
            }
        }
    }
}
