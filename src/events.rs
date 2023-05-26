use bevy::prelude::*;
use bevy_mod_picking::prelude::{Click, ListenedEvent};
use hexx::Hex;
use tracing::instrument;

use crate::{cats::Cat, players::PlayerId};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NextPlayer>();
        app.add_event::<ResetGameEvent>();
        app.add_event::<NewCat>();
        app.add_event::<MoveCat>();
        app.add_event::<GridCellClicked>();
    }
}

#[derive(Debug)]
pub struct ResetGameEvent;

#[derive(Debug)]
pub struct NewCat {
    pub player: PlayerId,
    pub cat: Cat,
    pub cell: Entity,
    pub position: Hex,
}

#[derive(Debug)]
pub struct MoveCat {
    pub from: Entity,
    pub to: Option<Hex>,
}

#[derive(Debug)]
pub struct GridCellClicked {
    pub cell: Entity,
}

impl From<ListenedEvent<Click>> for GridCellClicked {
    #[instrument(level = "debug", skip_all)]
    fn from(event: ListenedEvent<Click>) -> Self {
        GridCellClicked { cell: event.target }
    }
}

#[derive(Debug)]
pub struct SwitchPlayerEvent;

#[derive(Debug)]
pub struct NextPlayer;
