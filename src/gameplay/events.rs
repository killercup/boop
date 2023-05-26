use bevy::prelude::*;

use crate::cats::Cat;

#[derive(Debug)]
pub struct ResetGameEvent;

#[derive(Debug)]
pub struct NewCat {
    pub cat: Cat,
    pub cell: Entity,
}

#[derive(Debug)]
pub struct MoveCat {
    pub from: Entity,
    pub to: Entity,
}
