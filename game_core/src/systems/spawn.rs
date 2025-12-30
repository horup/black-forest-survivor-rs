use crate::{Entity, EntityVariant};
use super::Ctx;

pub fn spawn_system(spawn_event: &crate::event::SpawnEvent, ctx: &mut dyn Ctx) {
    let id = ctx.world_mut().entities.insert(Entity {
        pos: spawn_event.pos,
        variant: spawn_event.variant,
        move_dir: Default::default(),
        texture: Default::default(),
        facing: 0.0,
        solid: true,
        radius: 0.4,
        sprite_size: glam::Vec2::new(1.0, 1.0),
        ability_activates_at_sec: 0.5,
        ability_timer_sec: 0.0,
        ability_timer_total_sec: 1.0,
        move_sinus: 0.0,
        move_distance_total: 0.0,
    });

    match spawn_event.variant {
        EntityVariant::Player => {
            // set player entity id
            ctx.world_mut().player = id;
        },
        EntityVariant::Tree => {
            let w = ctx.rand_f32_range(1.0, 1.3);
            let h = ctx.rand_f32_range(1.5, 2.5);
            let e = ctx.world_mut().entity_mut(id).unwrap();
            e.radius = 0.1;
            e.sprite_size = glam::Vec2::new(w, h);
        },
        EntityVariant::Zombie => {
            
        }
        EntityVariant::Unknown => {}
    }
}
