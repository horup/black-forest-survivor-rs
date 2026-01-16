use glam::Vec4;

use crate::{TickEvent, systems::Ctx};

pub fn tick_system(event:&TickEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    for (_, entity) in world.entities.iter_mut() {
        entity.flash_timer.tick(event.dt);
        if entity.flash_timer.finished() {
            entity.tint = Vec4::ONE;
        } else {
            entity.tint = Vec4::new(0.75, 0.0, 0.0, 1.0);
        }
    }
}