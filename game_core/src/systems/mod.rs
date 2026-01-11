use crate::{Frame, Texture, World, event::Event, systems::{ability_hit::ability_hit_system, render::render_system}};

mod collision;
mod generate_map;
mod input;
mod map_entities;
mod movement;
mod restart;
mod spawn;
mod render;
mod despawn;
mod bot;
mod ability_hit;
mod ability_activated;
mod ability_cooldown;


pub use ability_activated::ability_activated_system;
pub use ability_cooldown::ability_cooldown_system;
pub use collision::collision_system;
pub use generate_map::generate_map_system;
pub use input::input_system;
pub use map_entities::map_entities_to_tiles_system;
pub use movement::movement_system;
pub use restart::restart_system;
pub use spawn::spawn_system;
pub use despawn::despawn_system;
pub use bot::bot_system;

use glam::{Vec2, Vec3, Vec4};

pub trait Ctx {
    fn world_mut(&mut self) -> &mut World;
    fn push_event(&mut self, event: Event) {
        self.world_mut().events.push_back(event);
    }
    fn rand_u32(&mut self) -> u32;
    fn rand_unsigned(&mut self, max: u32) -> u32 {
        self.rand_u32() % max
    }
    fn rand_f32(&mut self) -> f32 {
        self.rand_u32() as f32 / u32::MAX as f32
    }
    fn rand_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.rand_f32()
    }
    fn draw_tile(&mut self, origin:Vec3, texture:Texture, frame:Frame, color: Vec4);
    fn draw_sprite(&mut self, origin:Vec3, texture:Texture, frame:Frame, color:Vec4, scale:Vec2);
    /// Draws a full-screen flash with the specified color (RGBA, values 0.0-1.0).
    /// The alpha channel controls the opacity of the flash.
    fn draw_flash(&mut self, color: Vec4);
    
    /// Draws text at a world position
    fn draw_text(&mut self, origin:Vec3, text:String, color:Vec4);
}

pub fn process(ctx: &mut dyn Ctx) {
    while let Some(event) = ctx.world_mut().events.pop_front() {
        match event {
            Event::Tick(tick_event) => {
                generate_map_system(&tick_event, ctx);
                map_entities_to_tiles_system(&tick_event, ctx);
                bot_system(&tick_event, ctx);
                movement_system(&tick_event, ctx);
                ability_cooldown_system(&tick_event, ctx);
            }
            Event::Collision(collision_event) => {
                collision_system(&collision_event, ctx);
            }
            Event::Restart(restart_event) => {
                restart_system(&restart_event, ctx);
            }
            Event::Spawn(spawn_event) => {
                spawn_system(&spawn_event, ctx);
            }
            Event::PlayerInput(player_input_event) => {
                input_system(&player_input_event, ctx);
            },
            Event::AbilityActived(ability_actived_event) => {
                ability_activated_system(&ability_actived_event, ctx);
            },
            Event::PostTick(tick_event) => {
                render_system(&tick_event, ctx);
            },
            Event::Despawn(despawn_event) => {
                despawn_system(&despawn_event, ctx);
            },
            Event::AbilityHit(ability_hit_event) => {
                ability_hit_system(&ability_hit_event, ctx);
            },
        }
    }

    
}
