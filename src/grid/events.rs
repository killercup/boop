use bevy::prelude::*;
use bevy_mod_picking::prelude::{Click, ListenedEvent};
use tracing::instrument;

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
