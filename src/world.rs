use glam::Vec3;

#[derive(Clone, Copy)]
pub enum ThingVariant {
    Unknown,
    Player,
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
    /// The players entity id
    pub player: slotmap::DefaultKey,
}

impl World {
    pub fn clear(&mut self) {
        self.things.clear();
        self.tiles = Default::default();
    }

    pub fn player_mut(&mut self) -> Option<&mut Thing> {
        self.things.get_mut(self.player)
    }

    pub fn player(&self) -> Option<&Thing> {
        self.things.get(self.player)
    }
}