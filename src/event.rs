use glam::{Vec2, Vec3};
use slotmap::DefaultKey;

use crate::ThingVariant;

pub enum Event {
    Tick(TickEvent),
    Collision(CollisionEvent),
    Restart(RestartEvent),
    Spawn(SpawnEvent),
    PlayerInput(PlayerInputEvent),
}

pub struct PlayerInputEvent {
    pub player_id: DefaultKey,
    
    /// directional movement input
    /// normalized from 0..1
    pub move_dir: Vec3,

    /// facing direction in radians
    pub facing: f32,
}

pub struct TickEvent {
    pub dt:f32,
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