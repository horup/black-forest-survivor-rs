use glam::Vec4;

use crate::{Ctx, TickEvent, World};

pub fn render_system(_:&TickEvent, ctx: &mut dyn Ctx) {
    let Some(player) = ctx.world_mut().player() else { return; };
    let player_index = player.tile_index();
    let player_pos = player.pos;

    let draw_radius = World::draw_radius();
    let view_radius = World::view_radius();

    // draw tiles around player
    for y in -draw_radius.ceil() as i32..=draw_radius.ceil() as i32 {
        for x in -draw_radius.ceil() as i32..=draw_radius.ceil() as i32 {
            let cell = player_index + glam::IVec2::new(x, y);
            if let Some(tile) = ctx.world_mut().tiles.get(cell) {
                let origin = glam::Vec3::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5, 0.0);
                let c = 1.0;
                let color = Vec4::new(c, c, c, c);
                ctx.draw_tile(origin, "grass", color);
            }
        }
    }


}