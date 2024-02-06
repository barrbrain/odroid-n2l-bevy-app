//! A blank canvas for getting started on the ODROID-N2L and ODROID-VU5A.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use std::f32::consts::PI;

use crate::Cell::{Cross, Empty, Nought};
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
        .insert_resource(GameState::default())
        .add_system(bevy::window::close_on_esc)
        .add_system(touch_system)
        .run();
}

fn touch_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut targets: Query<(&Transform, &Target)>,
    mut game_state: ResMut<GameState>,
    touches: Res<Touches>,
) {
    for touch in touches.iter_just_pressed() {
        info!(
            "just pressed touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );

        let position = touch.position();
        let translation = Vec3::new(position.x - 400.0, 240.0 - position.y, 2.0);

        for (transform, target) in targets.iter() {
            if let Some(collision) = collide(
                transform.translation,
                transform.scale.truncate(),
                translation,
                Vec2::splat(1.0),
            ) {
                if !game_state.place(target.0) {
                    // Ignore invalid placement
                    continue;
                }
                let mut translation = transform.translation;
                translation.z = 2.0;
                if game_state.turn {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(45.).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::rgb(0.5, 0.5, 1.0))),
                        transform: Transform::from_translation(translation),
                        ..default()
                    });
                    translation.z = 3.0;
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(30.).into()).into(),
                        material: materials.add(ColorMaterial::from(BACKGROUND_COLOR)),
                        transform: Transform::from_translation(translation),
                        ..default()
                    });
                } else {
                    let scale = Vec3::new(15.0, 100.0, 1.0);
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 0.5, 0.5),
                            ..default()
                        },
                        transform: Transform {
                            translation,
                            rotation: Quat::from_rotation_z(PI * 0.25),
                            scale,
                        },
                        ..default()
                    });
                    translation.z = 3.0;
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 0.5, 0.5),
                            ..default()
                        },
                        transform: Transform {
                            translation,
                            rotation: Quat::from_rotation_z(PI * 0.75),
                            scale,
                        },
                        ..default()
                    });
                }
                game_state.turn ^= true;
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Rectangle.Vertical.Left
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(10.0, 350.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-60., 0., 0.)),
        ..default()
    });
    // Rectangle.Vertical.Right
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(10.0, 350.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(60., 0., 0.)),
        ..default()
    });
    // Rectangle.Horizontal.Top
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(350.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 60., 0.)),
        ..default()
    });
    // Rectangle.Horizontal.Bottom
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(350.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., -60., 0.)),
        ..default()
    });
    let mut index = 0;
    for y in (-120..=120).step_by(120) {
        for x in (-120..=120).step_by(120) {
            commands.spawn((target(Vec3::new(x as f32, y as f32, 0.)), Target(index)));
            index += 1;
        }
    }
}

#[derive(Default, Eq, PartialEq)]
enum Cell {
    #[default]
    Empty,
    Nought,
    Cross,
}

#[derive(Resource, Default)]
struct GameState {
    turn: bool,
    cells: [[Cell; 3]; 3],
}

impl GameState {
    fn place(&mut self, cell: usize) -> bool {
        let (row, column) = (cell / 3, cell % 3);
        if self.cells[row][column] != Empty {
            return false;
        }
        self.cells[row][column] = if self.turn { Cross } else { Nought };
        true
    }
}

#[derive(Component)]
struct Target(usize);
fn target(translation: Vec3) -> SpriteBundle {
    let scale = Vec3::new(110.0, 110.0, 1.0);

    SpriteBundle {
        sprite: Sprite {
            color: BACKGROUND_COLOR,
            ..default()
        },
        transform: Transform {
            translation,
            scale,
            ..default()
        },
        ..default()
    }
}
