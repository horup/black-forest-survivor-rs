mod systems;
pub use systems::*;
mod world;
pub use world::*;
mod event;
pub use event::*;
mod render;
pub use render::*;
mod entity;
pub use entity::*;
mod tile;
pub use tile::*;

use ggsdk::{
    GGAtlas, GGRunOptions, egui::Key
};
use glam::{Vec2, Vec3};
use glox::{FirstPersonCamera, Glox};

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
        render::render_ui(&g);
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
        self.world.events.push_back(Event::PlayerInput(PlayerInputEvent {
            player_id: self.world.player,
            move_dir: move_dir.normalize_or_zero(),
            facing,
            use_ability, 
        }));
        self.world.events.push_back(Event::Tick(TickEvent { dt: g.dt }));
        systems::process(self);
    }

    fn paint_glow(&mut self, g: ggsdk::PaintGlowContext) {
        let Some(player) = self.world.entities.get(self.world.player) else {
            return;
        };
        let player_pos = player.pos;
        self.fps_camera.eye = player_pos + Vec3::new(0.0, 0.0, 0.5);
        
        render::render_3d_world(&self.world, &self.fps_camera, &mut self.glox, &g);
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
