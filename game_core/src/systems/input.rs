use crate::PlayerInputEvent;
use super::Ctx;

/// handles inputs for entities in the world
pub fn input_system(e: &PlayerInputEvent, ctx: &mut dyn Ctx) {
    if let Some(entity) = ctx.world_mut().entities.get_mut(e.player_id) {
        if entity.health.is_alive() == false {
            // do not process input from dead entity
            return;
        }
        entity.move_dir = e.move_dir;
        entity.facing = e.facing;
        if e.use_ability {
            entity.active_ability();
        }
    }
}
