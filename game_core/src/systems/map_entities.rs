use crate::TickEvent;
use super::Ctx;

pub fn map_entities_to_tiles_system(_: &TickEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();

    // clear all tile entity mappings
    for chunk in &mut world.tiles {
        for (_, tile) in chunk {
            tile.entities.clear();
        }
    }

    // map entities to tiles
    let mut entities = Vec::new();
    world.entities(&mut entities);
    for entity_id in entities {
        let Some(entity) = world.entity(entity_id) else {
            continue;
        };
        let tile_index = entity.tile_index();

        if let Some(tile) = world.tiles.get_mut(tile_index) {
            tile.entities.insert(entity_id, ());
        }
    }
}
