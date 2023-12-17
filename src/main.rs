//! A blank canvas for getting started on the ODROID-N2L and ODROID-VU5A.

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
#[cfg(not(debug_assertions))]
use bevy::window::WindowMode;

const TIME_STEP: f32 = 1.0 / 60.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (800., 480.).into(),
                        resizable: false,
                        #[cfg(not(debug_assertions))]
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .add(FrameTimeDiagnosticsPlugin::default())
                .add(LogDiagnosticsPlugin::default()),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
