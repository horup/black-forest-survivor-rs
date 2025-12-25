mod update;
use shufflebag::ShuffleBag;
pub use update::*;
mod world;
pub use world::*;
mod event;
pub use event::*;

use std::{cell::RefCell, collections::{HashMap, VecDeque}};

use ggsdk::{
    GGAtlas, GGPainter, GGRunOptions, egui::{self, Align2, Color32, FontId, Key, LayerId, Pos2, Rect}
};
use glam::{IVec2, Vec2, Vec3, Vec4};
use glow::HasContext;
use glox::{Camera, FirstPersonCamera, Glox};

#[derive(Default)]
struct App {
    pub glox: Glox,
    pub fps_camera: FirstPersonCamera,
    pub world: World,
}

impl Ctx for App {
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }
    
    fn rand_u32(&mut self) -> u32 {
        rand::random::<u32>()
    }
}

impl ggsdk::GGApp for App {
    fn init(&mut self, g: ggsdk::InitContext) {
        self.glox.init(g.gl);
        self.fps_camera.eye = Vec3::new(0.0, 0.0, 0.5);

        g.assets
            .load::<GGAtlas>("assets/textures/grass.png", "grass");
        g.assets
            .load::<GGAtlas>("assets/textures/torch.png", "torch");
        g.assets
            .load::<GGAtlas>("assets/textures/axe.png", "axe");
        g.assets
            .load::<GGAtlas>("assets/textures/tree.png", "tree");

        self.world.events.push_front(Event::Restart(RestartEvent {}));
    }

    fn update(&mut self, g: ggsdk::UpdateContext) {
        let painter = g.egui_ctx.layer_painter(LayerId::background());


        let Some(torch) = g.assets.get::<GGAtlas>("torch") else { return ;};
        let screen_size = g.egui_ctx.input(|i| i.content_rect().size());

        let h= screen_size.y;
        let w = h / 2.0;

        painter.atlas(&torch, 0, Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(w, screen_size.y)), Color32::WHITE);


        let Some(axe) = g.assets.get::<GGAtlas>("axe") else { return ;};
        let screen_size = g.egui_ctx.input(|i| i.content_rect().size());
        let h= screen_size.y;
        let w = h / 2.0;
        painter.atlas(&axe, 0, Rect::from_min_max(Pos2::new(screen_size.x - w, 0.0), Pos2::new(screen_size.x, screen_size.y)), Color32::WHITE);
    }

    fn update_glow(&mut self, g: ggsdk::UpdateContext) {
        let mut move_dir = Vec2::new(0.0, 0.0);
        let mut pointer_delta = Vec2::new(0.0, 0.0);
        g.egui_ctx.input(|x| {
            let r = x.content_rect();
            self.fps_camera.viewport_size = Vec2::new(r.width(), r.height());

            if x.key_down(Key::W) {
                move_dir.y = 1.0;
            }
            if x.key_down(Key::S) {
                move_dir.y = -1.0;
            }
            if x.key_down(Key::A) {
                move_dir.x = -1.0;
            }
            if x.key_down(Key::D) {
                move_dir.x = 1.0;
            }

            let delta = x.pointer.motion().unwrap_or_default();
            pointer_delta = Vec2::new(delta.x, delta.y);

            let spd = 10.0;
            if x.key_down(Key::Q) {
                pointer_delta.x -= spd;
            }
            if x.key_down(Key::E) {
                pointer_delta.x += spd;
            }
        });

        let current_camera_pos = self.fps_camera.eye;
        self.fps_camera.move_self_horizontal(move_dir.extend(0.0));
        let new_camera_pos = self.fps_camera.eye;
        self.fps_camera.change_yaw(-pointer_delta.x / 100.0);
        let facing = self.fps_camera.yaw();
        let move_dir = new_camera_pos - current_camera_pos;
        self.world.events.push_back(Event::PlayerInput(PlayerInputEvent {
            player_id: self.world.player,
            move_dir: move_dir.normalize_or_zero(),
            facing,
        }));
        self.world.events.push_back(Event::Tick(TickEvent { dt: g.dt }));
        update::process(self);
    }

    fn paint_glow(&mut self, g: ggsdk::PaintGlowContext) {
        let Some(player) = self.world.things.get(self.world.player) else {
            return;
        };
        self.fps_camera.eye = player.pos + Vec3::new(0.0, 0.0, 0.5);
        let player_tile_pos = player.tile_pos();
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
        let size = 8;
        draw.bind_texture(Some(texture));
        for y in -size..size {
            for x in -size..size {
                let tile_index = player_tile_pos + IVec2::new(x, y);
                if let Some(_) = self.world.tiles.get(tile_index) {
                    let cell = player_tile_pos + Vec2::new(x as f32, y as f32).as_ivec2();
                    let p = Vec3::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5, 0.0);

                    let d = p - player.pos;
                    let d = d.length();
                    let d = d / size as f32;
                    let d = 1.0 - d;
                    let color = Vec4::new(d, d, d, 1.0);
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
                    "tree"
                }
                
            };
            let scaling_factor = match thing.variant {
                ThingVariant::Unknown => Vec2::new(1.0, 2.0),
                _ => {
                    Vec2::splat(1.0)
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
                scaling_factor,
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
            window_initial_size: Some((1280.0, 720.0)),
            ..Default::default()
        },
    );
}
