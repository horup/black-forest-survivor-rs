use crate::{DespawnEvent, systems::Ctx};

pub fn despawn_system(event: &DespawnEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    if world.entities.remove(event.entity_id).is_some() {
        dbg!("Despawned entity:", event.entity_id);
    }
}