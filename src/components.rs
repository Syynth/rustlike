pub use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Enemy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingRandomly;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

// #[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct Name(pub String);
