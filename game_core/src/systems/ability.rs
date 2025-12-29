use crate::{AbilityActivedEvent, TickEvent, event::Event};
use super::Ctx;

pub fn ability_activated_system(_: &AbilityActivedEvent, _ctx: &mut dyn Ctx) {
    dbg!("ability activated!");
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
