use crate::{ApplyDamageEvent, EntityVariant, Frame, systems::Ctx};

pub fn damage_system(event: &ApplyDamageEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    if let Some(entity) = world.entity_mut(event.entity_id) {
        if entity.health.can_receive_damage {
            entity.health.current -= event.damage_amount;
            if entity.is_ability_in_progress() {
                entity.reset_ability();
                entity.frame = Frame::Default;
            }

            if entity.health.current <= 0.0 {
                entity.health.current = 0.0;
                entity.solid = false;
                entity.max_speed = 0.0;
                entity.variant = EntityVariant::Unknown;
                entity.frame = Frame::Dead;
            }
        }
    }
}
