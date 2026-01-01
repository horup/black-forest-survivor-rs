use glam::{Vec2, Vec3, Vec3Swizzles};

use crate::{AbilityActivedEvent, DespawnEvent, TickEvent, event::Event, math};
use super::Ctx;

pub fn ability_activated_system(event: &AbilityActivedEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
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
                world.events.push_back(Event::Despawn(DespawnEvent { entity_id: other_entity_id }));
            }
        }
    }
}

pub fn ability_cooldown_system(tick_event: &TickEvent, ctx: &mut dyn Ctx) {
    let dt = tick_event.dt;
    let world = ctx.world_mut();
    let mut entities = Vec::new();
    world.entities(&mut entities);
    for entity_id in entities {
        if let Some(e) = world.entities.get_mut(entity_id) {
            if e.ability_timer_sec > 0.0 {
                let ability_timer_before_sec = e.ability_timer_sec;
                e.ability_timer_sec -= dt;
                if ability_timer_before_sec > e.ability_activates_at_sec && e.ability_timer_sec <= e.ability_activates_at_sec {
                    world.events.push_back(Event::AbilityActived(AbilityActivedEvent {
                        entity_id,
                    }));
                }
                if e.ability_timer_sec < 0.0 {
                    e.ability_timer_sec = 0.0;
                }
            }
        }
    }
}
