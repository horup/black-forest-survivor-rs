use crate::{TickEvent, Tile, event::Event};
use super::Ctx;

pub fn generate_map_system(_: &TickEvent, ctx: &mut dyn Ctx) {
    let player_id = ctx.world_mut().player;
    if let Some(player) = ctx.world_mut().entities.get_mut(player_id) {
        let grid_pos = player.pos.truncate().as_ivec2();
        let s = 16;
        for y in -s..=s {
            for x in -s..=s {
                let cell = grid_pos + glam::IVec2::new(x, y);
                if ctx.world_mut().tiles.get(cell).is_none() {
                    ctx.world_mut().tiles.insert(cell, Tile { solid: false, entities: Default::default() });
                    let r = ctx.rand_unsigned(6);
                    if r == 0 {
                        // spawn a tree
                        ctx.push_event(Event::Spawn(crate::event::SpawnEvent {
                            pos: glam::Vec3::new(
                                cell.x as f32 + 0.5,
                                cell.y as f32 + 0.5,
                                0.0,
                            ),
                            variant: crate::EntityVariant::Tree,
                        }));
                    } else if r == 1 {
                        // spawn a zombie
                        ctx.push_event(Event::Spawn(crate::event::SpawnEvent {
                            pos: glam::Vec3::new(
                                cell.x as f32 + 0.5,
                                cell.y as f32 + 0.5,
                                0.0,
                            ),
                            variant: crate::EntityVariant::Zombie,
                        }));    
                    }
                }
            }
        }
    }
}
