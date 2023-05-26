use bevy::{prelude::*, utils::HashMap};
use hexx::Hex;

pub mod events;
mod setup;

pub struct HexGridPlugin;

impl Plugin for HexGridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Map>();
        app.register_type::<MapSettings>();
        app.register_type::<GridCell>();
        app.register_type::<Grid>();

        app.add_event::<events::GridCellClicked>();

        app.init_resource::<MapSettings>();
        app.add_startup_system(setup::setup_grid);
    }
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Map {
    /// Hex grid
    cells: HashMap<Hex, Entity>,
    /// cats
    cats: HashMap<Hex, Option<Entity>>,
}

impl Map {
    pub fn cell_by_hex(&self, hex: Hex) -> Option<Entity> {
        self.cells.get(&hex).copied()
    }

    pub fn cell_by_entity(&self, entity: Entity) -> Option<Hex> {
        self.cells
            .iter()
            .find_map(|(hex, e)| if *e == entity { Some(*hex) } else { None })
    }

    pub fn cat_by_hex(&self, hex: Hex) -> Option<Entity> {
        self.cats.get(&hex).copied().flatten()
    }

    pub fn cat_by_entity(&self, entity: Entity) -> Option<Hex> {
        self.cats.iter().find_map(|(hex, e)| match e {
            Some(x) if *x == entity => Some(*hex),
            _ => None,
        })
    }

    pub fn add_cat(&mut self, hex: Hex, cat: Entity) {
        self.cats.insert(hex, Some(cat));
    }

    pub fn clear_cat_cell(&mut self, hex: Hex) {
        self.cats.remove(&hex);
    }
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

#[derive(Debug, Default, Resource)]
pub struct MapMaterials {
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct MapSettings {
    /// World size of the cells (outer radius)
    pub cell_size: Vec2,
    /// World space height of grid cells
    pub column_height: f32,
    /// Map radius
    pub map_radius: u32,
}

impl Default for MapSettings {
    fn default() -> Self {
        Self {
            cell_size: Vec2::splat(3.0),
            column_height: 1.0,
            map_radius: 4,
        }
    }
}
