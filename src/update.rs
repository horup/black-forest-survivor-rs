use std::collections::VecDeque;

use crate::{CollisionEvent, TickEvent, World, event::Event};

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
    dbg!("input system dt: {}", tick_event.dt);
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
    dbg!("collision system between {} and {}", collision_event.entity_1_id, collision_event.entity_2_id);
}

pub fn process_events(events: &mut VecDeque<Event>, world: &mut World) {
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
                input_system(&tick_event, &mut ctx);
                movement_system(&tick_event, &mut ctx);
            },
            Event::Collision(collision_event) => {
                collision_system(&collision_event, &mut ctx);
            },
        }
    }
}
