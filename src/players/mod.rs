use bevy::prelude::*;
use std::fmt;
use tracing::instrument;

mod plugin;
pub use plugin::*;

use crate::cats::Cat;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect, FromReflect, Component)]
#[reflect(Component)]
pub struct PlayerId(pub u8);

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

#[derive(Debug, Clone, PartialEq, Resource, Reflect)]
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
                    name: "Green".into(),
                    inventory: Inventory::default(),
                    color: Color::LIME_GREEN,
                },
                Player {
                    id: PlayerId::new(1),
                    name: "Orange".into(),
                    inventory: Inventory::default(),
                    color: Color::ORANGE,
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

    #[instrument(level = "debug", skip_all)]
    pub fn next_player(&mut self) -> &Player {
        let next_player = (self.current_player + 1).rem_euclid(self.players.len());
        if self.players[self.current_player].can_do_turn() {
            self.current_player = next_player;
        } else {
            debug!("Next player has no playable slots");
        }
        &self.players[self.current_player]
    }

    pub fn by_id(&self, id: PlayerId) -> Option<&Player> {
        self.players.iter().find(|p| p.id == id)
    }

    #[instrument(level = "debug", skip_all)]
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

    #[instrument(level = "debug", skip_all)]
    pub fn gain_kitten(&mut self, player: PlayerId) {
        let mut player = self.players.iter_mut().find(|p| p.id == player).unwrap();
        debug!("More kittens!");
        player.inventory.kittens += 1;
    }

    #[instrument(level = "debug", skip_all)]
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

    #[instrument(level = "debug", skip_all)]
    pub fn gain_cats(&mut self, num: u8) {
        let mut player = &mut self.players[self.current_player];
        debug!(num, "More cats!");
        player.inventory.cats += num;
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub inventory: Inventory,
    pub color: Color,
}

impl Player {
    pub fn can_do_turn(&self) -> bool {
        self.inventory.kittens + self.inventory.cats > 0
    }
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
