use glam::Vec2;

pub enum Event {
    Tick(TickEvent),
    Collision(CollisionEvent),
}

pub struct TickEvent {
    pub dt:f32,
    pub d_pad:Vec2
}

pub struct CollisionEvent {
    pub entity_1_id: u32,
    pub entity_2_id: u32,
}