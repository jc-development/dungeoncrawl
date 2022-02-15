use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
  ecs: &mut SubWorld,
  #[resource] map: &Map,
  #[resource] key: &Option<VirtualKeyCode>,
  #[resource] camera: &mut Camera
) {}