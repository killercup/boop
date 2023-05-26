use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[reflect(Component)]
pub enum Cat {
    #[default]
    Kitten,
    Adult,
}
