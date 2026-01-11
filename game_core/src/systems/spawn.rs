use crate::{Entity, EntityVariant, Health};
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
        max_speed: 0.0,
        floating_text: None,
        frame: Default::default(),
        move_sinus_speed: 1.0,
        health: Health::indistructible()
    });

    match spawn_event.variant {
        EntityVariant::Player => {
            // set player entity id
            ctx.world_mut().player = id;
            let e = ctx.world_mut().entity_mut(id).unwrap();
            e.ability_timer_total_sec = 0.5;
            e.ability_activates_at_sec = 0.2;
            e.max_speed = 2.5;
            e.move_sinus_speed = 2.5;
            e.health = Health {
                current: 100.0,
                max: 100.0,
                can_receive_damage: true,
            }
        },
        EntityVariant::Tree => {
            let w = ctx.rand_f32_range(1.0, 1.3);
            let h = ctx.rand_f32_range(1.5, 2.5);
            let e = ctx.world_mut().entity_mut(id).unwrap();
            e.radius = 0.1;
            e.texture = crate::Texture::Tree1;
            e.sprite_size = glam::Vec2::new(w, h);
            e.floating_text = Some("Tree".to_string());
        },
        EntityVariant::Zombie => {
            let e = ctx.world_mut().entity_mut(id).unwrap();
            e.texture = crate::Texture::Zombie1;
            e.sprite_size = glam::Vec2::new(0.5, 1.0);
            e.max_speed = 0.5;
            e.move_sinus_speed = 20.0;
            e.floating_text = Some("Zombie".to_string());
            e.health = Health {
                current: 30.0,
                max: 30.0,
                can_receive_damage: true,
            }
        }
        EntityVariant::Unknown => {}
    }
}
