use glam::{Vec2, Vec3};

use crate::ThingVariant;

pub enum Event {
    Tick(TickEvent),
    Collision(CollisionEvent),
    Restart(RestartEvent),
    Spawn(SpawnEvent),
}

pub struct TickEvent {
    pub dt:f32,
    pub d_pad:Vec2
}

pub struct CollisionEvent {
    pub entity_1_id: u32,
    pub entity_2_id: u32,
}

pub struct RestartEvent {

}

pub struct SpawnEvent {
    pub pos:Vec3,
    pub variant:ThingVariant,
}