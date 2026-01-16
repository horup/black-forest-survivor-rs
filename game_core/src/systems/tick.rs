use crate::{TickEvent, systems::Ctx};

pub fn tick_system(event:&TickEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    for (_, entity) in world.entities.iter_mut() {
        entity.flash_timer.tick(event.dt);
    }
}