use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub enum Cat {
    #[default]
    Kitten,
    Adult,
}

impl Cat {
    pub fn can_boop(&self, other: Cat) -> bool {
        matches!((self, other), (Cat::Adult, _) | (Cat::Kitten, Cat::Kitten))
    }
}

pub struct CatPlugin;

impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Cat>();
    }
}
