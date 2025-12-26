use ggsdk::egui::ahash::HashMap;
use slotmap::DefaultKey;

#[derive(Clone, Default)]
pub struct Tile {
    /// Whether this tile is solid (i.e., impassable)
    pub solid: bool,

    /// List of entity IDs currently occupying this tile
    pub entities: HashMap<DefaultKey, ()>,
}
