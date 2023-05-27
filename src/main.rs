use bevy::{log::LogPlugin, prelude::*};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    install_tracing(cfg!(debug_assertions));

    let mut app = App::new();
    app.add_plugins({
        let x = DefaultPlugins;
        let x = x.set(bevy_mod_picking::low_latency_window_plugin());
        let x = x.set(WindowPlugin {
            primary_window: Some(Window {
                title: "B⬡⬡P".to_string(), // ToDo
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        });
        #[cfg(not(target_arch = "wasm32"))]
        let x = x.disable::<LogPlugin>();

        x
    });

    #[cfg(feature = "dev")]
    {
        app.add_plugin(bevy_editor_pls::EditorPlugin::default());
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
    }

    app.add_plugin(boop::GamePlugin);

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
