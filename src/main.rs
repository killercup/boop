use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(low_latency_window_plugin()));
    app.add_plugins(DefaultPickingPlugins);

    #[cfg(feature = "dev")]
    app.add_plugin(bevy_editor_pls::EditorPlugin::default());

    app.add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), RaycastPickCamera::default()));
}
