use glam::Vec3;
use slotmap::DefaultKey;

use crate::entity::EntityVariant;

#[derive(Clone)]
pub enum Event {
    Tick(TickEvent),
    PostTick(TickEvent),
    Collision(CollisionEvent),
    Restart(RestartEvent),
    Spawn(SpawnEvent),
    Despawn(DespawnEvent),
    PlayerInput(PlayerInputEvent),
    AbilityActived(AbilityActivedEvent),
    AbilityHit(AbilityHitEvent),
    DamageEntity(ApplyDamageEvent),
}

#[derive(Clone)]
pub struct ApplyDamageEvent {
    pub entity_id: DefaultKey,
    pub other_entity_id: DefaultKey,
    pub damage_amount: f32,
}

#[derive(Clone)]
pub struct PlayerInputEvent {
    pub player_id: DefaultKey,
    
    /// directional movement input
    /// normalized from 0..1
    pub move_dir: Vec3,

    /// facing direction in radians
    pub facing: f32,

    /// whether the player used their ability this tick
    pub use_ability: bool,
}

#[derive(Clone)]
pub struct TickEvent {
    pub dt:f32,
}

#[derive(Clone)]
pub struct CollisionEvent {
    pub entity_1_id: DefaultKey,
    pub entity_2_id: DefaultKey,
}

#[derive(Clone)]
pub struct RestartEvent {

}

#[derive(Clone)]
pub struct SpawnEvent {
    pub pos:Vec3,
    pub variant:EntityVariant,
}

#[derive(Clone)]
pub struct AbilityActivedEvent {
    pub entity_id: DefaultKey,
}

#[derive(Clone)]
pub struct AbilityHitEvent {
    pub entity_id: DefaultKey,
    pub target_entity_id: DefaultKey,
}

#[derive(Clone)]
pub struct DespawnEvent {
    pub entity_id: DefaultKey,
}