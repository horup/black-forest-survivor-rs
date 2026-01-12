use glam::Vec3;
use crate::{EntityVariant, TickEvent};
use super::Ctx;

/// Bot AI system for zombies
/// - Finds the player's position
/// - Moves towards the player
/// - Activates ability when in contact range with the player
pub fn bot_system(_tick_event: &TickEvent, ctx: &mut dyn Ctx) {
    let world = ctx.world_mut();
    
    // Get player position
    let player_pos = if let Some(player) = world.player() {
        player.pos
    } else {
        return; // No player, no AI
    };

    // Collect all zombie entity IDs
    let mut zombie_ids = Vec::new();
    for (entity_id, entity) in world.entities.iter() {
        if matches!(entity.variant, EntityVariant::Zombie) {
            zombie_ids.push(entity_id);
        }
    }

    // Process each zombie
    for zombie_id in zombie_ids {
        let Some(zombie) = world.entities.get_mut(zombie_id) else {
            continue;
        };
        if zombie.health.is_alive() == false {
            continue; // Dead zombies do not act
        }

        let zombie_pos = zombie.pos;
        let zombie_radius = zombie.radius;
        
        // Calculate direction to player
        let direction = player_pos - zombie_pos;
        let distance = direction.length();

        // Check if zombie is in contact range with player (touching)
        let contact_range = zombie_radius + 0.4; // zombie radius + player radius (0.4)
        if distance <= contact_range {
            // Activate ability when touching player
            zombie.active_ability();
            // Stop moving when in contact
            zombie.move_dir = Vec3::ZERO;
        } else if distance > 0.0 {
            // Move towards player
            let normalized_dir = direction / distance;
            zombie.move_dir = normalized_dir;
            
            // Update facing direction
            zombie.facing = normalized_dir.y.atan2(normalized_dir.x);
        }
    }
}
