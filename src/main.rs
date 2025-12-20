mod update;
pub use update::*;
mod world;
pub use world::*;
mod event;
pub use event::*;

use std::collections::{HashMap, VecDeque};

use ggsdk::{
    GGAtlas, GGRunOptions,
    egui::{self, Align2, Color32, FontId, Key, LayerId},
};
use glam::{Vec2, Vec3, Vec4};
use glow::HasContext;
use glox::{Camera, FirstPersonCamera, Glox};

#[derive(Default)]
struct App {
    pub glox: Glox,
    pub fps_camera: FirstPersonCamera,
    pub world: World,
    pub events: VecDeque<Event>,
}

impl ggsdk::GGApp for App {
    fn init(&mut self, g: ggsdk::InitContext) {
        self.glox.init(g.gl);
        self.fps_camera.eye = Vec3::new(0.0, 0.0, 0.5);

        g.assets
            .load::<GGAtlas>("assets/textures/grass.png", "grass");

        self.events.push_front(Event::Restart(RestartEvent {  }));
    }

    fn update(&mut self, g: ggsdk::UpdateContext) {
        let painter = g.egui_ctx.layer_painter(LayerId::background());
    }

    fn update_glow(&mut self, g: ggsdk::UpdateContext) {
        let mut d_pad = Vec2::new(0.0, 0.0);
        let mut rot = 0.0;
        let mut pointer_delta = Vec2::new(0.0, 0.0);
        g.egui_ctx.input(|x| {
            let r = x.content_rect();
            self.fps_camera.viewport_size = Vec2::new(r.width(), r.height());

            if x.key_down(Key::W) {
                d_pad.y = 1.0;
            }
            if x.key_down(Key::S) {
                d_pad.y = -1.0;
            }
            if x.key_down(Key::A) {
                d_pad.x = -1.0;
            }
            if x.key_down(Key::D) {
                d_pad.x = 1.0;
            }
            if x.key_down(Key::Q) {
                rot = -1.0;
            }
            if x.key_down(Key::E) {
                rot = 1.0;
            }

            let delta = x.pointer.motion().unwrap_or_default();
            pointer_delta = Vec2::new(delta.x, delta.y);
        });

        self.events.push_back(Event::Tick(TickEvent { dt: g.dt, d_pad }));
        update::process(&mut self.events, &mut self.world);
    }

    fn paint_glow(&mut self, g: ggsdk::PaintGlowContext) {
        if let Some(player) = self.world.things.get(self.world.player) {
            self.fps_camera.eye = player.pos + Vec3::new(0.0, 0.0, 0.5);
        }
        let camera: &dyn Camera = &self.fps_camera;
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

        // draw tile
        let mut draw = self.glox.draw_builder(gl, camera);
        let player_pos = Vec2::default().as_ivec2();
        let size = 8;
        draw.bind_texture(Some(texture));
        for y in -size / 2..size / 2 {
            for x in -size / 2..size / 2 {
                if let Some(tile) = self.world.tiles.get(player_pos + Vec2::new(x as f32, y as f32).as_ivec2()) {
                    let cell = player_pos + Vec2::new(x as f32, y as f32).as_ivec2();
                    let p = Vec3::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5, 0.0);
                    let color = Vec4::new(1.0, 1.0, 1.0, 1.0);
                    draw.push_vertices(&glox::floor_vertices(p, color));
                }
            }
        }
        draw.finish();

        // draw some sprites / billboards
        for thing in self.world.things.values() {
            let mut draw = self.glox.draw_builder(gl, camera);
            let texture = match thing.variant {
                ThingVariant::Player => "grass",
                _ => {
                    continue;
                }
            };
            if let Some(atlas) = g.assets.get::<GGAtlas>(texture) {
                let texture = g.painter.texture(atlas.texture_id()).unwrap();
                draw.bind_texture(texture.into());
            }
            let p = thing.pos;
            draw.push_vertices(&glox::billboard_vertices(
                p,
                Vec4::splat(1.0),
                camera_dir,
                Vec2::splat(1.0),
            ));
            draw.finish();
        }

        self.glox.swap();
    }
}

fn sat_test(pos1: &Vec3, half_size: f32, pos2: &Vec3, half_size2: f32) -> bool {
    let delta = *pos2 - *pos1;
    let overlap_x = half_size + half_size2 - delta.x.abs();
    let overlap_y = half_size + half_size2 - delta.y.abs();
    let overlap_z = half_size + half_size2 - delta.z.abs();

    if overlap_x > 0.0 && overlap_y > 0.0 && overlap_z > 0.0 {
        true
    } else {
        false
    }
}

fn collision_resolve(pos1: &mut Vec3, half_size1: f32, pos2: &Vec3, half_size2: f32) {
    if !sat_test(pos1, half_size1, pos2, half_size2) {
        return;
    }
    let delta = *pos2 - *pos1;
    let overlap_x = half_size1 + half_size2 - delta.x.abs();
    let overlap_y = half_size1 + half_size2 - delta.y.abs();
    // Z is ignored for collision resolution

    if overlap_x < overlap_y {
        if delta.x > 0.0 {
            pos1.x -= overlap_x;
        } else {
            pos1.x += overlap_x;
        }
    } else {
        if delta.y > 0.0 {
            pos1.y -= overlap_y;
        } else {
            pos1.y += overlap_y;
        }
    }
}

fn main() {
    let app = App::default();
    ggsdk::GGEngine::run(
        app,
        GGRunOptions {
            ..Default::default()
        },
    );
}
