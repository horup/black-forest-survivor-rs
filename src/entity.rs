use glam::{Vec2, Vec3};

#[derive(Clone, Copy)]
pub enum EntityVariant {
    Unknown,
    Player,
    Tree
}

impl Default for EntityVariant {
    fn default() -> Self {
        EntityVariant::Unknown
    }
}

#[derive(Default, Clone, Copy)]
pub struct Entity {
    /// Position of this entity
    pub pos: glam::Vec3,

    /// Variant of this entity, such as a player, monster, item, etc. 
    pub variant: EntityVariant,

    /// Movement direction of this entity
    /// Normalized from 0..1
    pub move_dir: Vec3,

    /// Facing direction in radians
    pub facing: f32,

    /// Whether this entity is solid (i.e., impassable)
    pub solid: bool,

    /// Interaction radius, for collision detection
    pub radius:f32,

    /// Size of the sprite representing this entity
    pub sprite_size:Vec2,

    /// Ability cooldown timer in seconds
    pub ability_timer_sec:f32,

    /// Total duration of the ability cooldown in seconds
    pub ability_timer_total_sec:f32,

    /// Time when the ability actives
    pub ability_activates_at_sec:f32,
}

impl Entity {
    pub fn tile_index(&self) -> glam::IVec2 {
        glam::IVec2::new(self.pos.x as i32, self.pos.y as i32)
    }

    /// Activate the entity's ability, starting its cooldown timer
    /// If the ability is already on cooldown, this has no effect
    pub fn active_ability(&mut self) {
        if self.ability_timer_sec <= 0.0 {
            self.ability_timer_sec = self.ability_timer_total_sec;
        }
    }

    /// Returns a value between 0.0 and 1.0 representing the progress of the ability cooldown
    pub fn ability_delta(&self) -> f32 {
        if self.ability_timer_total_sec <= 0.0 {
            0.0
        } else {
            1.0 - (self.ability_timer_sec / self.ability_timer_total_sec)
        }
    }
}
