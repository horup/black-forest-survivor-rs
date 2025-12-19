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

impl World {
    pub fn test_world() -> Self {
        let mut world = World::default();

        // Create a simple test world with some solid and non-solid tiles
        for x in -10..10 {
            for y in -10..10 {
                let solid = (x + y) % 2 == 0; // Checkerboard pattern
                world.tiles.insert((x, y), Tile { solid });
            }
        }

        world.things.insert(Thing { pos: Vec3::default(), variant: ThingVariant::Player });
        world
    }
}