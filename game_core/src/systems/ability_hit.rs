use crate::{Frame};
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
            target_entity.frame = Frame::Dead;
            target_entity.solid = false;
            target_entity.max_speed = 0.0;
            target_entity.variant = crate::EntityVariant::Unknown;
        }   

        if ctx.world_mut().player == target_entity_id {
            ctx.draw_flash(glam::Vec4::new(1.0, 0.0, 0.0, 0.5));
        }
    }
}
