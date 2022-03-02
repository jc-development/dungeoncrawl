pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
  pub color: ColorPair,
  pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

// message component, storing a reference to an entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
  pub entity: Entity,
  pub destination: Point
}