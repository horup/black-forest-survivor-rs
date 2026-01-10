use std::collections::VecDeque;

use game_core::systems::Ctx;
pub use game_core::*;
use glow::HasContext;
mod render;
pub use render::*;

use ggsdk::{
    GGAtlas, GGRunOptions,
    egui::{Key, TextureId},
};
use glam::{Vec2, Vec3, Vec4};
use glox::{Camera, FirstPersonCamera, Glox};

#[derive(Default)]
struct App {
    pub glox: Glox,
    pub fps_camera: FirstPersonCamera,
    pub world: World,
    pub command_queue: VecDeque<AppCommand>,
    pub flash_color: Option<Vec4>,
    pub text_commands: Vec<(Vec3, String, Vec4)>,
}

enum AppCommand {
    DrawTile {
        origin: Vec3,
        texture: &'static str,
        color: Vec4,
    },
    DrawSprite {
        origin: Vec3,
        texture: &'static str,
        color: Vec4,
        scale: Vec2,
    },
    DrawFlash {
        color: Vec4,
    },
    DrawText {
        origin: Vec3,
        text: String,
        color: Vec4,
    },
}

fn texture_to_string(texture: Texture, frame: Frame) -> &'static str {
    match texture {
        Texture::None => Default::default(),
        Texture::Tree1 => "tree",
        Texture::Zombie1 => match frame {
            Frame::Default => "zombie_0",
            Frame::Walk1 => "zombie_1",
            Frame::Walk2 => "zombie_2",
            Frame::ReadyAttack => "zombie_3",
            Frame::Attack => "zombie_4",
        },
        Texture::Grass => "grass",
    }
}

impl Ctx for App {
    fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    fn rand_u32(&mut self) -> u32 {
        rand::random::<u32>()
    }

    fn draw_tile(&mut self, origin: Vec3, texture: Texture, frame: Frame, color: Vec4) {
        self.command_queue.push_back(AppCommand::DrawTile {
            origin,
            texture: texture_to_string(texture, frame),
            color,
        });
    }

    fn draw_sprite(
        &mut self,
        origin: Vec3,
        texture: Texture,
        frame: Frame,
        color: Vec4,
        scale: Vec2,
    ) {
        self.command_queue.push_back(AppCommand::DrawSprite {
            origin,
            texture: texture_to_string(texture, frame),
            color,
            scale,
        });
    }

    fn draw_flash(&mut self, color: Vec4) {
        self.command_queue
            .push_back(AppCommand::DrawFlash { color });
    }

    fn draw_text(&mut self, origin: Vec3, text: String, color: Vec4) {
        self.command_queue.push_back(AppCommand::DrawText {
            origin,
            text,
            color,
        });
    }
}

impl ggsdk::GGApp for App {
    fn init(&mut self, g: ggsdk::InitContext) {
        self.glox.init(g.gl);
        self.fps_camera.eye = Vec3::new(0.0, 0.0, 0.5);

        // Load all PNG textures from the textures directory
        // TODO make crossplatform
        if let Ok(entries) = std::fs::read_dir("assets/textures") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "png" {
                        if let (Some(path_str), Some(file_name)) =
                            (path.to_str(), path.file_stem().and_then(|s| s.to_str()))
                        {
                            let path_str = path_str.replace('\\', "/");
                            dbg!(&path_str);
                            dbg!(&file_name);

                            g.assets.load::<GGAtlas>(&path_str, file_name);
                        }
                    }
                }
            }
        }

        self.world
            .events
            .push_front(Event::Restart(RestartEvent {}));
    }

    fn update(&mut self, g: ggsdk::UpdateContext) {
        // Hide and capture the cursor
        g.egui_ctx.set_cursor_icon(ggsdk::egui::CursorIcon::None);
        g.egui_ctx
            .send_viewport_cmd(ggsdk::egui::ViewportCommand::CursorGrab(
                ggsdk::egui::CursorGrab::Confined,
            ));

        render::render_ui(&self.world, &g);

        // Render text commands that were extracted in paint_glow
        let camera: &dyn Camera = &self.fps_camera;
        for (origin, text, color) in self.text_commands.drain(..) {
            // Check if the text position is in front of the camera
            let to_point = origin - self.fps_camera.eye;
            let forward = self.fps_camera.direction();

            // Only render if dot product is positive (in front of camera)
            if to_point.dot(forward) > 0.0 {
                let screen_pos = camera.world_to_screen(origin);
                let painter = g.egui_ctx.layer_painter(ggsdk::egui::LayerId::background());
                let color32 = ggsdk::egui::Color32::from_rgba_premultiplied(
                    (color.x * 255.0) as u8,
                    (color.y * 255.0) as u8,
                    (color.z * 255.0) as u8,
                    (color.w * 255.0) as u8,
                );
                let pos = ggsdk::egui::Pos2::new(screen_pos.x, screen_pos.y);
                painter.text(
                    pos,
                    ggsdk::egui::Align2::CENTER_CENTER,
                    &text,
                    ggsdk::egui::FontId::proportional(20.0),
                    color32,
                );
            }
        }

        // Render flash overlay if present
        if let Some(color) = self.flash_color {
            let painter = g.egui_ctx.layer_painter(ggsdk::egui::LayerId::background());
            let screen_rect = g.egui_ctx.input(|i| i.content_rect());
            let color32 = ggsdk::egui::Color32::from_rgba_premultiplied(
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8,
                (color.w * 255.0) as u8,
            );
            painter.rect_filled(screen_rect, 0.0, color32);

            // Clear flash after rendering
            self.flash_color = None;
        }
    }

    fn update_glow(&mut self, g: ggsdk::UpdateContext) {
        let mut move_dir = Vec2::new(0.0, 0.0);
        let mut pointer_delta = Vec2::new(0.0, 0.0);
        let mut use_ability = false;
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
            if x.key_down(Key::Space) {
                use_ability = true;
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

            if x.pointer.primary_down() {
                use_ability = true;
            }
        });

        let current_camera_pos = self.fps_camera.eye;
        self.fps_camera.move_self_horizontal(move_dir.extend(0.0));
        let new_camera_pos = self.fps_camera.eye;
        self.fps_camera.change_yaw(-pointer_delta.x / 100.0);
        let facing = self.fps_camera.yaw();
        let move_dir = new_camera_pos - current_camera_pos;
        self.world
            .events
            .push_back(Event::PlayerInput(PlayerInputEvent {
                player_id: self.world.player,
                move_dir: move_dir.normalize_or_zero(),
                facing,
                use_ability,
            }));
        self.world
            .events
            .push_back(Event::Tick(TickEvent { dt: g.dt }));
        systems::process(self);
        self.world
            .events
            .push_back(Event::PostTick(TickEvent { dt: g.dt }));
        systems::process(self);

        // Synchronize camera with player position after systems have processed
        if let Some(player) = self.world.entities.get(self.world.player) {
            self.fps_camera.eye = player.pos + Vec3::new(0.0, 0.0, 0.5);
        }
    }

    fn paint_glow(&mut self, g: ggsdk::PaintGlowContext) {
        let Some(player) = self.world.entities.get(self.world.player) else {
            return;
        };
        let player_pos = player.pos;
        let camera_dir = self.fps_camera.direction();
        let gl = g.painter.gl();

        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }

        let mut current_texture_id: Option<TextureId> = None;
        let mut draw = self.glox.draw_builder(gl, &self.fps_camera);

        for command in self.command_queue.drain(..) {
            match command {
                AppCommand::DrawTile {
                    origin,
                    texture,
                    color,
                } => {
                    let Some(texture) = g.assets.get::<GGAtlas>(&texture) else {
                        continue;
                    };
                    let texture_id = texture.texture_id();

                    // If texture changed, finish current batch and start new one
                    if current_texture_id != Some(texture_id) {
                        if current_texture_id.is_some() {
                            draw.finish();
                            draw = self.glox.draw_builder(gl, &self.fps_camera);
                        }
                        let texture = g.painter.texture(texture_id).unwrap();
                        draw.bind_texture(Some(texture));
                        current_texture_id = Some(texture_id);
                    }

                    draw.push_vertices(&glox::floor_vertices(origin, color));
                }
                AppCommand::DrawSprite {
                    origin,
                    texture,
                    color,
                    scale,
                } => {
                    // Cull sprites behind the camera
                    let to_sprite = origin - player_pos;
                    if to_sprite.dot(camera_dir) <= 0.0 {
                        continue;
                    }

                    let Some(texture) = g.assets.get::<GGAtlas>(&texture) else {
                        continue;
                    };
                    let texture_id = texture.texture_id();

                    // If texture changed, finish current batch and start new one
                    if current_texture_id != Some(texture_id) {
                        if current_texture_id.is_some() {
                            draw.finish();
                            draw = self.glox.draw_builder(gl, &self.fps_camera);
                        }
                        let texture = g.painter.texture(texture_id).unwrap();
                        draw.bind_texture(Some(texture));
                        current_texture_id = Some(texture_id);
                    }

                    draw.push_vertices(&glox::billboard_vertices(origin, color, camera_dir, scale));
                }
                AppCommand::DrawFlash { color } => {
                    // Store flash color to be rendered in the update method
                    self.flash_color = Some(color);
                }
                AppCommand::DrawText {
                    origin,
                    text,
                    color,
                } => {
                    // Store text commands for rendering in update() where egui_ctx is available
                    self.text_commands.push((origin, text, color));
                }
            }
        }

        // Finish the final batch if any commands were processed
        if current_texture_id.is_some() {
            draw.finish();
        }

        self.glox.swap();
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
