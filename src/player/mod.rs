use bevy::prelude::*;

use crate::cats::Cat;

pub mod events;

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub id: u8,
    pub kittens: u8,
    pub cats: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            id: 0,
            kittens: 6,
            cats: 0,
        }
    }
}

impl Player {
    pub fn take_kitten(&mut self) -> Option<Cat> {
        if self.kittens > 0 {
            self.kittens -= 1;
            return Some(Cat::Kitten);
        }
        None
    }

    pub fn take_cat(&mut self) -> Option<Cat> {
        if self.cats > 0 {
            self.cats -= 1;
            return Some(Cat::Adult);
        }
        None
    }
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct CurrentPlayer {
    pub id: Entity,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
        // app.register_type::<CurrentPlayer>();

        app.add_event::<events::SwitchPlayerEvent>();

        app.add_startup_system(setup_player);
        app.add_system(switch_player.run_if(on_event::<events::SwitchPlayerEvent>()));
    }
}

fn setup_player(mut commands: Commands) {
    let red = commands
        .spawn((Player { id: 0, ..default() }, Name::from("Red")))
        .id();
    let _blue = commands
        .spawn((Player { id: 1, ..default() }, Name::from("Blue")))
        .id();

    commands.insert_resource(CurrentPlayer { id: red });
}

fn switch_player(mut current_player: ResMut<CurrentPlayer>, players: Query<Entity, With<Player>>) {
    for player in players.iter() {
        if player != current_player.id {
            current_player.id = player;
            debug!(?player, "New current player");
            break;
        }
    }
}
