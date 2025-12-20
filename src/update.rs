use std::collections::VecDeque;

use crate::{CollisionEvent, RestartEvent, TickEvent, Tile, World, event::Event};

pub struct Ctx<'a> {
    pub world: &'a mut World,
    push_event: &'a mut dyn FnMut(Event),
}

impl<'a> Ctx<'a> {
    pub fn push_event(&mut self, event: Event) {
        (self.push_event)(event);
    }
}

/// handles inputs from player
pub fn input_system(tick_event: &TickEvent, ctx: &mut Ctx) {
    if let Some(player) = ctx.world.player_mut() {
        let speed = 5.0;
        let forward = glam::Vec3::new(0.0, 1.0, 0.0);
        let right = glam::Vec3::new(1.0, 0.0, 0.0);
        let movement = (forward * tick_event.d_pad.y + right * tick_event.d_pad.x) * speed * tick_event.dt;
        player.pos += movement;
    }
}

/// handles movement of things in the world
pub fn movement_system(tick_event: &TickEvent, ctx: &mut Ctx) {
    dbg!("movement system dt: {}", tick_event.dt);
    ctx.push_event(Event::Collision(CollisionEvent {
        entity_1_id: 1,
        entity_2_id: 2,
    }));
}

pub fn collision_system(collision_event: &CollisionEvent, ctx: &mut Ctx) {
    dbg!(
        "collision system between {} and {}",
        collision_event.entity_1_id,
        collision_event.entity_2_id
    );
}

pub fn spawn_system(spawn_event: &crate::event::SpawnEvent, ctx: &mut Ctx) {
    let id = ctx.world.things.insert(crate::world::Thing {
        pos: spawn_event.pos,
        variant: spawn_event.variant,
    });

    match spawn_event.variant {
        crate::world::ThingVariant::Player => {
            // set player entity id
            ctx.world.player = id;
        }
        _ => {}
    }
}

pub fn restart_system(_: &RestartEvent, ctx: &mut Ctx) {
    ctx.world.clear();
    ctx.push_event(Event::Spawn(crate::event::SpawnEvent {
        pos: glam::Vec3::default(),
        variant: crate::world::ThingVariant::Player,
    }));
}

pub fn generate_map_system(_: &TickEvent, ctx: &mut Ctx) {
    if let Some(player) = ctx.world.things.get_mut(ctx.world.player) {
        let grid_pos = player.pos.truncate().as_ivec2();
        let s = 16;
        for y in -s..=s {
            for x in -s..=s {
                let cell = grid_pos + glam::IVec2::new(x, y);
                if ctx.world.tiles.get(cell).is_none() {
                    ctx.world.tiles.insert(cell, Tile { solid: false });
                }
            }
        }
    }
}

pub fn process(events: &mut VecDeque<Event>, world: &mut World) {
    while let Some(event) = events.pop_front() {
        let mut push_event = |e: Event| {
            events.push_back(e);
        };
        let mut ctx = Ctx {
            world,
            push_event: &mut push_event,
        };
        match event {
            Event::Tick(tick_event) => {
                generate_map_system(&tick_event, &mut ctx);
                input_system(&tick_event, &mut ctx);
                movement_system(&tick_event, &mut ctx);
            }
            Event::Collision(collision_event) => {
                collision_system(&collision_event, &mut ctx);
            }
            Event::Restart(restart_event) => {
                restart_system(&restart_event, &mut ctx);
            }
            Event::Spawn(spawn_event) => {
                spawn_system(&spawn_event, &mut ctx);
            }
        }
    }
}
