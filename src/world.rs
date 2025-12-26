use std::collections::VecDeque;

use glam::IVec2;
use slotmap::DefaultKey;

use crate::{Event, thing::Thing, tile::{self, Tile}};

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
    /// Clear the world of all things and tiles
    pub fn clear(&mut self) {
        self.things.clear();
        self.tiles = Default::default();
    }

    /// Get a mutable reference to the player thing
    pub fn player_mut(&mut self) -> Option<&mut Thing> {
        self.things.get_mut(self.player)
    }

    /// Get a reference to the player thing
    pub fn player(&self) -> Option<&Thing> {
        self.things.get(self.player)
    }

    /// Get all entities within a certain radius of a tile position
    pub fn get_entities(&self, tile_pos:IVec2, radius:f32, entities:&mut Vec<DefaultKey>) {
        let s = radius.ceil() as i32;
        for y in -s..=s {
            for x in -s..=s {
                let cell = tile_pos + glam::IVec2::new(x, y);
                if let Some(tile) = self.tiles.get(cell) {
                    for (entity_id, _) in tile.entities.iter() {
                        entities.push(*entity_id);
                    }
                }
            }
        }
    }
}