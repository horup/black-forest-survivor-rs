use glam::Vec3;

#[derive(Clone, Copy)]
pub enum ThingVariant {
    Unknown,
    Player,
    Tree
}

impl Default for ThingVariant {
    fn default() -> Self {
        ThingVariant::Unknown
    }
}

#[derive(Default, Clone, Copy)]
pub struct Thing {
    /// Position of this thing
    pub pos: glam::Vec3,

    /// Variant of this thing, such as a player, monster, item, etc. 
    pub variant: ThingVariant,

    /// Movement direction of this thing
    /// Normalized from 0..1
    pub move_dir: Vec3,

    /// Facing direction in radians
    pub facing: f32,
}

impl Thing {
    pub fn tile_pos(&self) -> glam::IVec2 {
        glam::IVec2::new(self.pos.x as i32, self.pos.y as i32)
    }
}
