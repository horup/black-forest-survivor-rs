use glam::{Vec2, Vec3};
use slotmap::DefaultKey;

use crate::thing::ThingVariant;

#[derive(Clone)]
pub enum Event {
    Tick(TickEvent),
    Collision(CollisionEvent),
    Restart(RestartEvent),
    Spawn(SpawnEvent),
    PlayerInput(PlayerInputEvent),
}

#[derive(Clone)]
pub struct PlayerInputEvent {
    pub player_id: DefaultKey,
    
    /// directional movement input
    /// normalized from 0..1
    pub move_dir: Vec3,

    /// facing direction in radians
    pub facing: f32,
}

#[derive(Clone)]
pub struct TickEvent {
    pub dt:f32,
}

#[derive(Clone)]
pub struct CollisionEvent {
    pub entity_1_id: u32,
    pub entity_2_id: u32,
}

#[derive(Clone)]
pub struct RestartEvent {

}

#[derive(Clone)]
pub struct SpawnEvent {
    pub pos:Vec3,
    pub variant:ThingVariant,
}