use glam::{Vec2, Vec3, Vec4};

use crate::{Frame, Texture, Timer};

#[derive(Clone, Copy)]
pub enum EntityVariant {
    Unknown,
    Player,
    Tree,
    Zombie,
}

impl Default for EntityVariant {
    fn default() -> Self {
        EntityVariant::Unknown
    }
}

#[derive(Default, Clone)]
pub struct Entity {
    /// Position of this entity
    pub pos: glam::Vec3,

    /// Variant of this entity, such as a player, monster, item, etc. 
    pub variant: EntityVariant,

    /// Texture used to represent this entity
    pub texture: Texture,

    /// Current animation frame of this entity
    pub frame: Frame,

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

    /// sinus value for movement animation
    /// used for e.g. bobbing up and down while walking
    pub move_sinus:f32,

    /// speed of the sinus movement
    pub move_sinus_speed:f32,

    /// total distance moved
    /// reset when move_sinus is reset
    pub move_distance_total:f32,

    /// Maximum movement speed of this entity
    pub max_speed: f32,

    /// Optional text to display floating over the entity
    pub floating_text: Option<String>,

    /// Health of this entity
    pub health: Health,

    /// Color tint applied to this entity
    pub tint: Vec4,

    /// Timer for flash effects (e.g., when taking damage)
    pub flash_timer:Timer
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

    /// Resets the ability cooldown, making it immediately available
    pub fn reset_ability(&mut self) {
        self.ability_timer_sec = 0.0;
    }

    /// Returns a value between 0.0 and 1.0 representing the progress of the ability cooldown
    pub fn ability_delta(&self) -> f32 {
        if self.ability_timer_total_sec <= 0.0 {
            0.0
        } else {
            1.0 - (self.ability_timer_sec / self.ability_timer_total_sec)
        }
    }

    /// Returns true if the ability is currently in progress
    pub fn is_ability_in_progress(&self) -> bool {
        self.ability_timer_sec > 0.0
    }
}

#[derive(Clone)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub can_receive_damage: bool,
}

impl Default for Health {
    fn default() -> Self {
        Health {
            current: 100.0,
            max: 100.0,
            can_receive_damage: true,
        }
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Health {
            current: max,
            max,
            can_receive_damage: true,
        }
    }

    pub fn indistructible() -> Self {
        Health {
            current: 0.0,
            max: 0.0,
            can_receive_damage: false,
        }
    }

    pub fn apply_damage(&mut self, damage: f32) {
        if self.can_receive_damage {
            self.current -= damage;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}