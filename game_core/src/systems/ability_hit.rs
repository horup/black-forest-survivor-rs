use crate::{Frame, event::Event};
use super::Ctx;

pub fn ability_hit_system(event: &crate::AbilityHitEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    let target_entity_id = event.target_entity_id;
    if let Some(target_entity) = world.entities.get_mut(target_entity_id) {
        target_entity.health.apply_damage(30.0);
        dbg!(target_entity.health.current);
        if target_entity.is_ability_in_progress() {
            target_entity.reset_ability();
            target_entity.frame = Frame::Default;
        }

        if target_entity.health.is_alive() == false {
            world.events.push_back(Event::Despawn(crate::DespawnEvent { entity_id: target_entity_id }));
        }   
    }
}
