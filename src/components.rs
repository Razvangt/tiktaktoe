use bevy::prelude::Component;

#[derive(Component)]
pub struct BoardGame;
#[derive(Debug, Copy, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}
#[derive(Debug, Copy, Clone)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct TurnMovement;
