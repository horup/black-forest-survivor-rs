use std::collections::VecDeque;

use crate::{CollisionEvent, PlayerInputEvent, RestartEvent, Thing, TickEvent, Tile, World, event::Event};

pub struct Ctx<'a> {
    pub world: &'a mut World,
    push_event: &'a mut dyn FnMut(Event),
}

impl<'a> Ctx<'a> {
    pub fn push_event(&mut self, event: Event) {
        (self.push_event)(event);
    }
}

/// handles inputs for things in the world
pub fn input_system(e: &PlayerInputEvent, ctx: &mut Ctx) {
    if let Some(thing) = ctx.world.things.get_mut(e.player_id) {
        thing.move_dir = e.move_dir;
        thing.facing = e.facing;
    }
}

/// handles movement of things in the world
pub fn movement_system(tick_event: &TickEvent, ctx: &mut Ctx) {
    let dt = tick_event.dt;

    let max_speed = 5.0;
    for (_id, thing) in ctx.world.things.iter_mut() {
        thing.pos += thing.move_dir * dt * max_speed;
    }
    ctx.push_event(Event::Collision(CollisionEvent {
        entity_1_id: 1,
        entity_2_id: 2,
    }));
}

pub fn collision_system(collision_event: &CollisionEvent, ctx: &mut Ctx) {
   
}

pub fn spawn_system(spawn_event: &crate::event::SpawnEvent, ctx: &mut Ctx) {
    let id = ctx.world.things.insert(Thing {
        pos: spawn_event.pos,
        variant: spawn_event.variant,
        move_dir: Default::default(),
        facing: 0.0,
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
                    dbg!("some tile");
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
            Event::PlayerInput(player_input_event) => {
                input_system(&player_input_event, &mut ctx);
            },
        }
    }
}
