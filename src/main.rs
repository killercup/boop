use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(low_latency_window_plugin()));
    app.add_plugins(DefaultPickingPlugins);

    app.insert_resource(AmbientLight {
        brightness: 0.1,
        ..default()
    });
    app.add_startup_system(setup);

    app.add_plugin(boop::grid::HexGridPlugin);
    app.add_plugin(boop::player::PlayerPlugin);
    app.add_plugin(boop::gameplay::GamePlayPlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(bevy_editor_pls::EditorPlugin::default());

    app.add_system(reset_game);

    app.run();
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y);
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

fn reset_game(
    keys: Res<Input<KeyCode>>,
    mut reset_grid: EventWriter<boop::gameplay::events::ResetGameEvent>,
    // mut reset_players: EventWriter<boop::ResetPlayers>,
) {
    if keys.just_pressed(KeyCode::R) {
        reset_grid.send(boop::gameplay::events::ResetGameEvent);
        // reset_players.send(boop::ResetPlayers);
    }
}
