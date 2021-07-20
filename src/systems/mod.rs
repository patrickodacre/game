use crate::prelude::*;

mod collisions;
mod entity_render;
mod map_render;
mod monster_ai;
mod player_input;

pub fn build_scheduler() -> Schedule
{
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(monster_ai::monster_ai_system())
        .add_system(collisions::collisions_system())
        .build()
}
