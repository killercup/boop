use bevy::prelude::*;
use hexx::Hex;

use crate::cats::Cat;

#[derive(Debug)]
pub struct ResetGameEvent;

#[derive(Debug)]
pub struct NewCat {
    pub cat: Cat,
    pub cell: Entity,
    pub position: Hex,
}

#[derive(Debug)]
pub struct MoveCat {
    pub from: Entity,
    pub to: Option<Entity>,
}
