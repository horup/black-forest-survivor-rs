use std::collections::VecDeque;

use crate::{Event, thing::Thing, tile::Tile};

#[derive(Default, Clone)]
pub struct World {
    /// All things in the game world
    pub things: slotmap::SlotMap<slotmap::DefaultKey, Thing>,
    /// The tiles making up the game world
    pub tiles: endlessgrid::Grid<Tile>,
    /// The players entity id
    pub player: slotmap::DefaultKey,

    /// Unprocessed events
    pub events:VecDeque<Event>
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