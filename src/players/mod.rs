use bevy::prelude::*;
use std::fmt;

mod plugin;
pub use plugin::*;

use crate::cats::Cat;

pub mod events;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, FromReflect)]
pub struct PlayerId(u8);

impl PlayerId {
    pub fn new(id: u8) -> PlayerId {
        PlayerId(id)
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub struct Players {
    players: Vec<Player>,
    current_player: usize,
}

impl Default for Players {
    fn default() -> Self {
        Self {
            players: vec![
                Player {
                    id: PlayerId::new(0),
                    inventory: Inventory::default(),
                },
                Player {
                    id: PlayerId::new(1),
                    inventory: Inventory::default(),
                },
            ],
            current_player: 0,
        }
    }
}

impl Players {
    pub fn current(&self) -> &Player {
        &self.players[self.current_player]
    }

    pub fn next_player(&mut self) -> &Player {
        self.current_player = (self.current_player + 1).rem_euclid(self.players.len());
        &self.players[self.current_player]
    }

    pub fn from_id(&self, id: PlayerId) -> &Player {
        self.players
            .iter()
            .find(|p| p.id == id)
            .expect("valid player id")
    }

    pub fn take_kitten(&mut self) -> Option<Cat> {
        let mut player = &mut self.players[self.current_player];
        if player.inventory.kittens > 0 {
            player.inventory.kittens -= 1;
            Some(Cat::Kitten)
        } else {
            debug!("No more kittens");
            None
        }
    }

    pub fn take_cat(&mut self) -> Option<Cat> {
        let mut player = &mut self.players[self.current_player];
        if player.inventory.cats > 0 {
            player.inventory.cats -= 1;
            Some(Cat::Adult)
        } else {
            debug!("No more cats");
            None
        }
    }

    pub fn gain_cats(&mut self, num: u8) {
        let mut player = &mut self.players[self.current_player];
        debug!(num, "More cats!");
        player.inventory.cats += num;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect, FromReflect)]
pub struct Player {
    pub id: PlayerId,
    pub inventory: Inventory,
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect, FromReflect)]
pub struct Inventory {
    cats: u8,
    kittens: u8,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            cats: 0,
            kittens: 6,
        }
    }
}
