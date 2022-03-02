///
/// The movement system iterates all entities with a WantsToMove component.
/// It then checks that the move is valid, and if true, replaces the Point
/// component of the target entity.
/// 
/// If the entity is a player, it also updates the camera.
/// 

use crate::prelude::*;

/// Legion provides shorthand for systems that only run a single query.
/// Derives the query from the system params - and runs the system once
/// for every matching entity.
/// This is the same as declaring a query that reads Entity and WantsToMove
/// and iterating it as you have with other systems.
#[system(for_each)]

#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}
