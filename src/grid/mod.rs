use bevy::{prelude::*, utils::HashMap};
use hexx::Hex;

pub mod events;
mod setup;

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Map>();
        app.register_type::<MapSettings>();

        app.add_event::<events::GridCellClicked>();

        app.init_resource::<MapSettings>();
        app.add_startup_system(setup::setup_grid);
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Map {
    /// Hex grid
    entities: HashMap<Hex, Entity>,
}

impl Map {
    pub fn cell_by_hex(&self, hex: Hex) -> Option<Entity> {
        self.entities.get(&hex).copied()
    }

    pub fn cell_by_entity(&self, entity: Entity) -> Option<Hex> {
        self.entities
            .iter()
            .find_map(|(hex, e)| if *e == entity { Some(*hex) } else { None })
    }
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Grid;

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub struct GridCell;

#[derive(Debug, Default, Resource)]
pub struct MapMaterials {
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct MapSettings {
    /// World size of the hexagons (outer radius)
    hex_size: Vec2,
    /// World space height of hex columns
    column_height: f32,
    /// Map radius
    map_radius: u32,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            hex_size: Vec2::splat(3.0),
            column_height: 1.0,
            map_radius: 4,
        }
    }
}
