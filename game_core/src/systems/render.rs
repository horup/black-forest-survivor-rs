use std::collections::HashMap;

use glam::{IVec2, Vec4};

use crate::{Fade, Frame, Texture, TickEvent, World, systems::Ctx};

pub fn render_system(event:&TickEvent, ctx: &mut dyn Ctx) {
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
            if let Some(_) = ctx.world_mut().tiles.get(cell) {
                let origin = glam::Vec3::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5, 0.0);
                let v = origin - player_pos;
                let c = World::light(v.length());
                let color = Vec4::new(c, c, c, c);
                ctx.draw_tile(origin, Texture::Grass, Frame::Default, color);
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
            let color = e.tint * c;
            let color = color.truncate().extend(1.0);
            let sprite_size = e.sprite_size;
            let texture = e.texture;
            let floating_text = e.floating_text.clone();
            let frame = e.frame;
            ctx.draw_sprite(origin, texture, frame, color, sprite_size);
            
            // Draw floating text if present
            if let Some(text) = floating_text {
                let text_pos = origin + glam::Vec3::new(0.0, 0.0, sprite_size.y * 1.0);
                ctx.draw_text(text_pos, text, Vec4::new(c, c, c, 1.0));
            }
        }
    }
    
    // Update and render fade effect
    ctx.world_mut().fade_timer.tick(event.dt);
    let alpha = match ctx.world_mut().fade {
        Fade::In => (1.0 - ctx.world_mut().fade_timer.progress()).clamp(0.0, 1.0),
        Fade::Out => ctx.world_mut().fade_timer.progress().clamp(0.0, 1.0),
    };
    ctx.draw_flash(Vec4::new(0.0, 0.0, 0.0, alpha));
}