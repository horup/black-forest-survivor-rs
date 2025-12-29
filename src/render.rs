use crate::{entity::EntityVariant, World};
use ggsdk::{GGAtlas, GGPainter, egui::{Color32, LayerId, Pos2, Rect}};
use glam::{IVec2, Vec2, Vec3, Vec4};
use glow::HasContext;
use glox::{Camera, FirstPersonCamera};

pub fn texture_for_entity_variant(
    variant: EntityVariant,
) -> &'static str {
    match variant {
        EntityVariant::Player => "player",
        EntityVariant::Tree => "tree",
        _ => "unknown",
    }
}

pub struct Render {
    
}

impl Render {
   
}

/// Renders the 2D UI elements (torch and axe)
pub fn render_ui(world:&World, g: &ggsdk::UpdateContext) {
    let painter = g.egui_ctx.layer_painter(LayerId::background());

    let Some(torch) = g.assets.get::<GGAtlas>("torch") else { return; };
    let screen_size = g.egui_ctx.input(|i| i.content_rect().size());

    let h = screen_size.y;
    let w = h / 2.0;

    let Some(player) = world.entities.get(world.player) else { return; };
    {
        let f = h / 20.0;
        let x = 0.0;//-player.move_sinus * f;
        let y = (player.move_sinus + 1.0) * f;
        painter.atlas(&torch, 0, Rect::from_min_max(Pos2::new(x, y), Pos2::new(w + x, screen_size.y + y)), Color32::WHITE);
    }

    let Some(axe) = g.assets.get::<GGAtlas>("axe") else { return; };

  
    let cooldown = player.ability_delta();
    if cooldown == 1.0 {
        // not on cooldown, draw normally
        let f = h / 20.0;
        let x = player.move_sinus * f;
        let y = h / 4.0 + player.move_sinus * f;
        painter.atlas(&axe, 0, Rect::from_min_max(Pos2::new(screen_size.x - w + x, y), Pos2::new(screen_size.x + x, screen_size.y + y)), Color32::WHITE);
    } else {
        let y = cooldown * 2.0 * h;
        let x = -cooldown * 3.0 *w;
        painter.atlas(&axe, 0, Rect::from_min_max(Pos2::new(screen_size.x - w + x, y), Pos2::new(screen_size.x + x, screen_size.y + y)), Color32::WHITE);
    }
   
}

/// Renders the 3D world using OpenGL
pub fn render_3d_world(
    world: &World,
    fps_camera: &FirstPersonCamera,
    glox: &mut glox::Glox,
    g: &ggsdk::PaintGlowContext,
) {
    let Some(player) = world.entities.get(world.player) else {
        return;
    };
    let player_pos = player.pos;
    let player_tile_pos = player.tile_index();
    let camera: &dyn Camera = fps_camera;
    let Some(texture) = g.assets.get::<GGAtlas>("grass") else {
        return;
    };

    let texture = g.painter.texture(texture.texture_id()).unwrap();
    let camera_dir = camera.direction();
    let gl = g.painter.gl();
    unsafe {
        gl.enable(glow::DEPTH_TEST);
        gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
    }

    let max_distance = 8;
    let light = |d: f32| {
        let d = d / max_distance as f32;
        let d = 1.0 - d;
        if d < 0.0 {
            0.0
        } else if d > 1.0 {
            1.0
        } else {
            d
        }
    };

    // draw tile
    let mut draw = glox.draw_builder(gl, camera);

    draw.bind_texture(Some(texture));
    for y in -max_distance..max_distance {
        for x in -max_distance..max_distance {
            let tile_index = player_tile_pos + IVec2::new(x, y);
            if let Some(_) = world.tiles.get(tile_index) {
                let cell = player_tile_pos + Vec2::new(x as f32, y as f32).as_ivec2();
                let p = Vec3::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5, 0.0);
                let d = p - player_pos;
                let d = d.length();
                let d = light(d);
                let color = Vec4::new(d, d, d, 1.0);
                draw.push_vertices(&glox::floor_vertices(p, color));
            }
        }
    }
    draw.finish();

    // draw some sprites / billboards
    for entity in world.entities.values() {
        let mut draw = glox.draw_builder(gl, camera);
        let texture = match entity.variant {
            EntityVariant::Player => "grass",
            _ => "tree"
        };
        let scaling_factor = entity.sprite_size;
        if let Some(atlas) = g.assets.get::<GGAtlas>(texture) {
            let texture = g.painter.texture(atlas.texture_id()).unwrap();
            draw.bind_texture(texture.into());
        }
        let p = entity.pos;
        let d = p - player_pos;
        let d = d.length();
        let d = light(d);
        let color = Vec4::new(d, d, d, 1.0);
        draw.push_vertices(&glox::billboard_vertices(
            p,
            color,
            camera_dir,
            scaling_factor,
        ));
        draw.finish();
    }

    glox.swap();
}