use std::collections::HashMap;

use crate::{AbilityActivedEvent, CollisionEvent, Entity, EntityVariant, PlayerInputEvent, RestartEvent, TickEvent, Tile, World, event::Event};

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
}


/// handles inputs for things in the world
pub fn input_system(e: &PlayerInputEvent, ctx: &mut dyn Ctx) {
    if let Some(thing) = ctx.world_mut().entities.get_mut(e.player_id) {
        thing.move_dir = e.move_dir;
        thing.facing = e.facing;
    }
}

/// handles movement of things in the world
/// also handled collision resolution
pub fn movement_system(tick_event: &TickEvent, ctx: &mut dyn Ctx) {
    let dt = tick_event.dt;
    let max_speed = 5.0;
    let world = ctx.world_mut();
    let mut entities = Vec::new();
    world.entities(&mut entities);
    let mut close_entities = Vec::new();
    let mut colliding_entities = HashMap::new();
    for entity_id in entities {
        close_entities.clear();
        colliding_entities.clear();
        let Some(thing) = world.entity(entity_id) else {
            continue;
        };
        let thing_vel = thing.move_dir * dt * max_speed;
        let thing_pos = thing.pos;
        let thing_solid = thing.solid;
        let thing_radius = thing.radius;
        let thing_tile_index = thing.tile_index();

        if thing_vel.length() == 0.0 {
            continue;
        }

        // first remove thing from current tile
        if let Some(tile) = world.tiles.get_mut(thing_tile_index) {
            tile.entities.remove(&entity_id);
        }

        // now move the thing
        let mut thing_pos = thing_pos + thing_vel;

        // check and resolve collision with solid things
        if thing_solid {
            world.get_entities(thing_pos.truncate().as_ivec2(), 2.0, &mut close_entities);
        }

        for other_entity_id in &close_entities {
            if *other_entity_id == entity_id {
                continue;
            }
            let Some(other_thing) = world.entity(*other_entity_id) else {
                continue;
            };
            if !other_thing.solid {
                continue;
            }

            let to_other = other_thing.pos - thing_pos;
            let dist = to_other.length();
            let min_dist = thing_radius + other_thing.radius;
            if dist < min_dist && dist > 0.0 {
                let overlap = min_dist - dist;
                let correction = to_other.normalize() * overlap;
                thing_pos += -correction * 0.5;
                colliding_entities.insert(*other_entity_id, ());
            }
        }

        // finally update thing position
        if let Some(thing_mut) = world.entities.get_mut(entity_id) {
            thing_mut.pos = thing_pos;
        }

        // add thing to new tile
        let new_tile_index = thing_pos.truncate().as_ivec2();
        if let Some(tile) = world.tiles.get_mut(new_tile_index) {
            tile.entities.insert(entity_id, ());
        }

        for other_entity_id in colliding_entities.keys() {
            world.events.push_back(Event::Collision(CollisionEvent {
                entity_1_id: entity_id.clone(),
                entity_2_id: other_entity_id.clone(),
            }));
        }
    }
}

pub fn collision_system(_collision_event: &CollisionEvent, _ctx: &mut dyn Ctx) {
   
}


pub fn map_entities_to_tiles_system(_: &TickEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();

    // clear all tile entity mappings
    for chunk in &mut world.tiles {
        for (_, tile) in chunk {
            tile.entities.clear();
        }
    }

    // map entities to tiles
    let mut entities = Vec::new();
    world.entities(&mut entities);
    for entity_id in entities {
        let Some(thing) = world.entity(entity_id) else {
            continue;
        };
        let tile_index = thing.tile_index();

        if let Some(tile) = world.tiles.get_mut(tile_index) {
            tile.entities.insert(entity_id, ());
        }
    }
}

pub fn spawn_system(spawn_event: &crate::event::SpawnEvent, ctx: &mut dyn Ctx) {
    let id = ctx.world_mut().entities.insert(Entity {
        pos: spawn_event.pos,
        variant: spawn_event.variant,
        move_dir: Default::default(),
        facing: 0.0,
        solid: true,
        radius: 0.4,
        sprite_size: glam::Vec2::new(1.0, 1.0),
        ability_activates_at_sec: 0.5,
        ability_timer_sec: 0.0,
        ability_timer_total_sec: 1.0
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
        _ => {}
    }
}

pub fn restart_system(_: &RestartEvent, ctx: &mut dyn Ctx) {
    ctx.world_mut().clear();
    ctx.push_event(Event::Spawn(crate::event::SpawnEvent {
        pos: glam::Vec3::default(),
        variant: crate::EntityVariant::Player,
    }));
}

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
                    }
                }
            }
        }
    }
}

pub fn ability_activated_system(_: &AbilityActivedEvent, _ctx: &mut dyn Ctx) {
    dbg!("ability activated!");
}

pub fn ability_cooldown_system(tick_event: &TickEvent, ctx: &mut dyn Ctx) {
    let dt = tick_event.dt;
    let world = ctx.world_mut();
    let mut entities = Vec::new();
    world.entities(&mut entities);
    for entity_id in entities {
        if let Some(thing) = world.entities.get_mut(entity_id) {
            if thing.ability_timer_sec > 0.0 {
                let ability_timer_before_sec = thing.ability_timer_sec;
                thing.ability_timer_sec -= dt;
                if ability_timer_before_sec < thing.ability_activates_at_sec && thing.ability_timer_sec >= thing.ability_activates_at_sec {
                    world.events.push_back(Event::AbilityActived(AbilityActivedEvent {
                        entity_id,
                    }));
                }
                if thing.ability_timer_sec < 0.0 {
                    thing.ability_timer_sec = 0.0;
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
                map_entities_to_tiles_system(&tick_event, ctx);
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
        }
    }
}
