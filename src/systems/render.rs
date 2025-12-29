use std::{collections::HashMap, f32::consts::E};

use glam::{IVec2, Vec4};

use crate::{Ctx, Entity, TickEvent, World};

pub fn render_system(_:&TickEvent, ctx: &mut dyn Ctx) {
    let Some(player) = ctx.world_mut().player() else { return; };
    let player_index = player.tile_index();
    let player_pos = player.pos;

    let draw_radius = World::draw_radius();

    let mut tiles:HashMap<IVec2, ()> = HashMap::new();
    let mut entities = Vec::new();

    // draw tiles around player
    for y in -draw_radius.ceil() as i32..=draw_radius.ceil() as i32 {
        for x in -draw_radius.ceil() as i32..=draw_radius.ceil() as i32 {
            let cell = player_index + glam::IVec2::new(x, y);
            if let Some(tile) = ctx.world_mut().tiles.get(cell) {
                let origin = glam::Vec3::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5, 0.0);
                let v = origin - player_pos;
                let c = World::light(v.length());
                let color = Vec4::new(c, c, c, c);
                ctx.draw_tile(origin, "grass", color);
                tiles.insert(cell, ());
            }
        }
    }

    // collect entities to draw based upon their tile position
    for (id, entity) in ctx.world_mut().entities.iter() {
        let tile_index = entity.tile_index();
        if tiles.contains_key(&tile_index) {
            entities.push(id);
        }
    }   

    // draw entities
    for e in entities.iter() {
        if let Some(e) = ctx.world_mut().entities.get(*e) {
            let origin = e.pos;
            let v = origin - player_pos;
            let d = v.length();
            let c = World::light(d);
            let color = Vec4::new(c, c, c, c);
            let sprite_size = e.sprite_size;
            ctx.draw_sprite(origin, "tree", color, sprite_size);
        }
    }
}