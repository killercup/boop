#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweeningPlugin;

mod cats;
mod events;
mod gameplay;
mod grid;
mod loading;
mod players;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();

        app.add_plugin(loading::LoadingPlugin);
        app.add_plugins(DefaultPickingPlugins);
        app.add_plugin(TweeningPlugin);

        app.add_plugin(events::EventsPlugin);
        app.add_plugin(cats::CatPlugin);
        app.add_plugin(grid::HexGridPlugin);
        app.add_plugin(players::PlayerPlugin);

        app.add_plugin(gameplay::GamePlayPlugin);

        app.add_startup_system(setup);
        app.add_system(reset_game);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 0.1,
        ..default()
    });
    let transform = Transform::from_xyz(0.0, 30.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform,
            ..default()
        },
        RaycastPickCamera::default(),
    ));
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
}

fn reset_game(keys: Res<Input<KeyCode>>, mut reset_grid: EventWriter<events::ResetGameEvent>) {
    if keys.just_pressed(KeyCode::R) {
        reset_grid.send(events::ResetGameEvent);
    }
}
