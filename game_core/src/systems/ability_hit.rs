use super::Ctx;

pub fn ability_hit_system(event: &crate::AbilityHitEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    let target_entity_id = event.target_entity_id;
    if let Some(_target_entity) = world.entities.get_mut(target_entity_id) {
        world.events.push_back(crate::Event::DamageEntity(crate::ApplyDamageEvent { entity_id: target_entity_id, other_entity_id: event.entity_id, damage_amount: 30.0 }));
    }
}
