use glam::Vec3;

use crate::{AbilityActivedEvent, TickEvent, event::Event};
use super::Ctx;

pub fn ability_activated_system(event: &AbilityActivedEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    if let Some(e) = world.entities.get_mut(event.entity_id) {
        let facing = e.facing;
        let pos = e.pos;

        // TODO: improve tihs such that it only targets entities in a cone in front of the activator
        // find entity in fron of the activator
        let target_pos = pos + Vec3::new(facing.cos(), facing.sin(), 0.0);
        let mut target_entity_id_opt = None;
        for (other_entity_id, other_entity) in world.entities.iter() {
            if other_entity_id == event.entity_id {
                continue;
            }
            let v = other_entity.pos - target_pos;
            let dist = v.length();
            if dist > 0.5 {
                continue;
            }
            target_entity_id_opt = Some(other_entity_id);
            break;
        }

        dbg!(target_entity_id_opt);
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
