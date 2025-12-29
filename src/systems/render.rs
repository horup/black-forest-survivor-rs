use crate::{Ctx, TickEvent};

pub fn render_system(_:&TickEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    dbg!("drawing 3d world");
}