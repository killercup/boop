use bevy::prelude::*;
use hexx::Hex;

use crate::{events::ResetGameEvent, players::Players, GameState};

use self::setup::{setup_grid, CellMaterials};

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
        app.add_system(highlight_cell.in_set(OnUpdate(GameState::Playing)));
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
pub struct Platform;

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct GridCell(pub Hex);

impl std::ops::Deref for GridCell {
    type Target = Hex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Hovered;

fn highlight_cell(
    players: Res<Players>,
    materials: Res<CellMaterials>,
    map: Res<Map>,
    mut hovered_cell: Query<
        (&mut Handle<StandardMaterial>, &GridCell),
        (With<Platform>, With<Hovered>),
    >,
    mut other_cells: Query<(&mut Handle<StandardMaterial>,), (With<Platform>, Without<Hovered>)>,
) {
    let player_material = materials.hovered_by_player[players.current().id.0 as usize].clone();
    hovered_cell.iter_mut().for_each(|(mut material, cell)| {
        if map.cat_by_hex(cell.0).is_none() {
            *material = player_material.clone();
        }
    });

    other_cells.iter_mut().for_each(|(mut material,)| {
        *material = materials.default.clone();
    });
}
