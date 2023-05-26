use bevy::{log::LogPlugin, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweeningPlugin;
use tracing_subscriber::fmt::format::FmtSpan;

use boop::events;

fn main() {
    install_tracing(cfg!(debug_assertions));

    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(low_latency_window_plugin())
            .disable::<LogPlugin>(),
    );
    app.add_plugins(DefaultPickingPlugins);
    app.add_plugin(TweeningPlugin);

    app.insert_resource(AmbientLight {
        brightness: 0.1,
        ..default()
    });

    app.add_plugin(boop::events::EventsPlugin);
    app.add_plugin(boop::cats::CatPlugin);
    app.add_plugin(boop::grid::HexGridPlugin);
    app.add_plugin(boop::players::PlayerPlugin);
    app.add_plugin(boop::gameplay::GamePlayPlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(bevy_editor_pls::EditorPlugin::default());

    app.add_startup_system(setup);
    app.add_system(reset_game);

    app.run();
}

fn install_tracing(verbose: bool) {
    use std::{env, io};
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let fmt_layer = fmt::layer()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_writer(io::stderr);
    let default = if verbose { "boop=debug" } else { "boop=info" }
        .parse()
        .unwrap();

    let filter_layer = if !env::var("RUST_LOG").map(|x| !x.is_empty()).unwrap_or(false) {
        EnvFilter::new("warn,wgpu_code=error")
    } else {
        EnvFilter::builder()
            .with_env_var("RUST_LOG")
            .from_env_lossy()
    };

    tracing_subscriber::registry()
        .with(filter_layer.add_directive(default))
        .with(fmt_layer)
        .init();
}

fn setup(mut commands: Commands) {
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
