use bevy::prelude::*;
use hexx::Hex;

use crate::{events::ResetGameEvent, GameState};

use self::setup::setup_grid;

mod map;
mod setup;

pub use map::{Map, MapSettings};

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Map>();
        app.register_type::<MapSettings>();
        app.register_type::<GridCell>();
        app.register_type::<Grid>();

        app.init_resource::<MapSettings>();
        app.init_resource::<Map>();

        app.add_system(setup_grid.in_schedule(OnExit(GameState::Loading)));
        app.add_system(reset_map.run_if(on_event::<ResetGameEvent>()));
    }
}

fn reset_map(mut map: ResMut<Map>) {
    map.cats = default();
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Grid;

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct GridCell(pub Hex);

impl std::ops::Deref for GridCell {
    type Target = Hex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
