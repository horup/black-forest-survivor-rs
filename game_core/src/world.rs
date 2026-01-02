use std::collections::VecDeque;

use glam::IVec2;
use slotmap::DefaultKey;

use crate::{Event, entity::Entity, tile::Tile};

#[derive(Default, Clone)]
pub struct World {
    /// All entities in the game world
    pub entities: slotmap::SlotMap<slotmap::DefaultKey, Entity>,
    /// The tiles making up the game world
    pub tiles: endlessgrid::Grid<Tile>,
    /// The players entity id
    pub player: slotmap::DefaultKey,
    /// Unprocessed events
    pub events:VecDeque<Event>,
    /// Time elapsed for fade-in effect (in seconds)
    pub fade_in_time: f32,
}

impl World {
    pub fn light(d:f32) -> f32 {
        let max_distance = Self::view_radius();
        
        // Inverse square falloff with smooth cutoff
        // This provides more realistic light attenuation
        let normalized_d = d / max_distance;
        
        if normalized_d >= 1.0 {
            return 0.0;
        }
        
        // Inverse square law: intensity = 1 / (1 + k*d^2)
        // k controls the falloff rate
        let k = 16.0;
        let attenuation = 1.0 / (1.0 + k * normalized_d * normalized_d);
        
        // Smooth cutoff near the edge to avoid hard boundaries
        let edge_softness = 0.2;
        if normalized_d > (1.0 - edge_softness) {
            let edge_factor = (1.0 - normalized_d) / edge_softness;
            attenuation * edge_factor
        } else {
            attenuation
        }
    }

    pub fn draw_radius() -> f32 {
        8.0
    }

    pub fn view_radius() -> f32 {
        8.0
    }

    /// Get all entity IDs in the world
    pub fn entities(&self, entities:&mut Vec<DefaultKey>) {
        for (entity_id, _) in self.entities.iter() {
            entities.push(entity_id);
        }
    }

    pub fn entity_mut(&mut self, entity_id:DefaultKey) -> Option<&mut Entity> {
        self.entities.get_mut(entity_id)
    }

    pub fn entity(&self, entity_id:DefaultKey) -> Option<&Entity> {
        self.entities.get(entity_id)
    }

    /// Clear the world of all entities and tiles
    pub fn clear(&mut self) {
        self.entities.clear();
        self.tiles = Default::default();
    }

    /// Get a mutable reference to the player entity
    pub fn player_mut(&mut self) -> Option<&mut Entity> {
        self.entities.get_mut(self.player)
    }

    /// Get a reference to the player entity
    pub fn player(&self) -> Option<&Entity> {
        self.entities.get(self.player)
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