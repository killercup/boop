use bevy::{prelude::*, utils::HashMap};
use hexx::Hex;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Map {
    /// Hex grid
    pub cells: HashMap<Hex, Entity>,
    /// cats
    pub cats: HashMap<Hex, Option<Entity>>,
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
            map_radius: 3,
        }
    }
}
