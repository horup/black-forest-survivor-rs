use crate::{RestartEvent, event::Event};
use super::Ctx;

pub fn restart_system(_: &RestartEvent, ctx: &mut dyn Ctx) {
    ctx.world_mut().clear();
    ctx.push_event(Event::Spawn(crate::event::SpawnEvent {
        pos: glam::Vec3::default(),
        variant: crate::EntityVariant::Player,
    }));
}
