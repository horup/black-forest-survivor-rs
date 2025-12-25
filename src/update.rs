use crate::{CollisionEvent, PlayerInputEvent, RestartEvent, Thing, TickEvent, Tile, World, event::Event};

pub trait Ctx {
    fn world_mut(&mut self) -> &mut World;
    fn push_event(&mut self, event: Event) {
        self.world_mut().events.push_back(event);
    }
    fn rand_u32(&mut self) -> u32;
    fn rand_unsigned(&mut self, max: u32) -> u32 {
        self.rand_u32() % max
    }
}


/// handles inputs for things in the world
pub fn input_system(e: &PlayerInputEvent, ctx: &mut dyn Ctx) {
    if let Some(thing) = ctx.world_mut().things.get_mut(e.player_id) {
        thing.move_dir = e.move_dir;
        thing.facing = e.facing;
    }
}

/// handles movement of things in the world
pub fn movement_system(tick_event: &TickEvent, ctx: &mut dyn Ctx) {
    let dt = tick_event.dt;

    let max_speed = 5.0;
    for (_id, thing) in ctx.world_mut().things.iter_mut() {
        thing.pos += thing.move_dir * dt * max_speed;
    }
    ctx.push_event(Event::Collision(CollisionEvent {
        entity_1_id: 1,
        entity_2_id: 2,
    }));
}

pub fn collision_system(_collision_event: &CollisionEvent, _ctx: &mut dyn Ctx) {
   
}

pub fn spawn_system(spawn_event: &crate::event::SpawnEvent, ctx: &mut dyn Ctx) {
    let id = ctx.world_mut().things.insert(Thing {
        pos: spawn_event.pos,
        variant: spawn_event.variant,
        move_dir: Default::default(),
        facing: 0.0,
    });

    match spawn_event.variant {
        crate::ThingVariant::Player => {
            // set player entity id
            ctx.world_mut().player = id;
        }
        _ => {}
    }
}

pub fn restart_system(_: &RestartEvent, ctx: &mut dyn Ctx) {
    ctx.world_mut().clear();
    ctx.push_event(Event::Spawn(crate::event::SpawnEvent {
        pos: glam::Vec3::default(),
        variant: crate::ThingVariant::Player,
    }));
}

pub fn generate_map_system(_: &TickEvent, ctx: &mut dyn Ctx) {
    let player_id = ctx.world_mut().player;
    if let Some(player) = ctx.world_mut().things.get_mut(player_id) {
        let grid_pos = player.pos.truncate().as_ivec2();
        let s = 16;
        for y in -s..=s {
            for x in -s..=s {
                let cell = grid_pos + glam::IVec2::new(x, y);
                if ctx.world_mut().tiles.get(cell).is_none() {
                    ctx.world_mut().tiles.insert(cell, Tile { solid: false });
                    let r = ctx.rand_unsigned(6);
                    if r == 0 {
                        // spawn a tree
                        ctx.push_event(Event::Spawn(crate::event::SpawnEvent {
                            pos: glam::Vec3::new(
                                cell.x as f32 + 0.5,
                                cell.y as f32 + 0.5,
                                0.0,
                            ),
                            variant: crate::ThingVariant::Tree,
                        }));
                    }
                }
            }
        }
    }
}

pub fn process(ctx: &mut dyn Ctx) {
    while let Some(event) = ctx.world_mut().events.pop_front() {
        match event {
            Event::Tick(tick_event) => {
                generate_map_system(&tick_event, ctx);
                movement_system(&tick_event, ctx);
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
        }
    }
}
