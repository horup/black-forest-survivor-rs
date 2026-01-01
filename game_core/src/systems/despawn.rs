use crate::{DespawnEvent, systems::Ctx};

pub fn despawn_system(event: &DespawnEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    
    // Get the entity's position before removing it
    if let Some(entity) = world.entity(event.entity_id) {
        let tile_pos = entity.pos.truncate().as_ivec2();
        
        // Remove entity from its tile
        if let Some(tile) = world.tiles.get_mut(tile_pos) {
            tile.entities.remove(&event.entity_id);
        }
    }
    
    // Remove entity from the world
    if world.entities.remove(event.entity_id).is_some() {
        dbg!("Despawned entity:", event.entity_id);
    }
}