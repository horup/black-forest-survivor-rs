#[derive(Clone, Copy)]
pub enum ThingVariant {
    Unknown
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
}

#[derive(Clone, Default)]
pub struct Tile {
    /// Whether this tile is solid (i.e., impassable)
    pub solid: bool,
}

#[derive(Default, Clone)]
pub struct World {
    /// All things in the game world
    pub things: slotmap::SlotMap<slotmap::DefaultKey, Thing>,
    /// The tiles making up the game world
    pub tiles: endlessgrid::Grid<Tile>,
}