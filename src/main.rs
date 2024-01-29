//! A blank canvas for getting started on the ODROID-N2L and ODROID-VU5A.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, diagnostic::LogDiagnosticsPlugin, prelude::*,
    sprite::MaterialMesh2dBundle,
};

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
        .add_system(mouse_click_system)
        .run();
}

fn mouse_click_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    touches: Res<Touches>,
) {
    for touch in touches.iter_just_pressed() {
        info!(
            "just pressed touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );

        let position = touch.position();
        let translation = Vec3::new(position.x - 400.0, 240.0 - position.y, 0.0);

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(translation),
            ..default()
        });
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(10.0, 300.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-60., 0., 0.)),
        ..default()
    });
    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(10.0, 300.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(60., 0., 0.)),
        ..default()
    });
    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(300.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 60., 0.)),
        ..default()
    });

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(300.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -60., 0.)),
        ..default()
    });
}
