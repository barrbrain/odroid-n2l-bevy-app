//! A blank canvas for getting started on the ODROID-N2L and ODROID-VU5A.

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use std::f32::consts::{PI, SQRT_2};

use crate::Cell::{Cross, Empty, Nought};
use crate::EndGame::{Column, Diagonal, Incomplete, Row, Tie};
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
        .add_system(close_button_system)
        .add_system(reset_button_system)
        .add_system(touch_system)
        .run();
}

fn touch_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
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
                if !game_state.turn {
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(45.).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::rgb(0.5, 0.5, 1.0))),
                        transform: Transform::from_translation(translation),
                        ..default()
                    });
                    translation.z = 3.0;
                    commands.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(25.).into()).into(),
                        material: materials.add(ColorMaterial::from(BACKGROUND_COLOR)),
                        transform: Transform::from_translation(translation),
                        ..default()
                    });
                } else {
                    let scale = Vec3::new(25.0, 100.0, 1.0);
                    spawn_cross(&mut commands, translation, scale, Color::rgb(1.0, 0.5, 0.5));
                }
                let color = if !game_state.turn {
                    Color::rgb(0.35, 0.35, 0.75)
                } else {
                    Color::rgb(0.75, 0.35, 0.35)
                };
                translation.z = 4.0;
                match game_state.end_game {
                    Row(_) => {
                        translation.x = 0.0;
                        commands.spawn(SpriteBundle {
                            sprite: Sprite { color, ..default() },
                            transform: Transform {
                                translation,
                                rotation: Quat::from_rotation_z(PI * 0.5),
                                scale: Vec3::new(15.0, 345.0, 1.0),
                            },
                            ..default()
                        });
                    }
                    Column(_) => {
                        translation.y = 0.0;
                        commands.spawn(SpriteBundle {
                            sprite: Sprite { color, ..default() },
                            transform: Transform {
                                translation,
                                rotation: Quat::from_rotation_z(PI * 0.0),
                                scale: Vec3::new(15.0, 345.0, 1.0),
                            },
                            ..default()
                        });
                    }
                    Diagonal(0) => {
                        translation.x = 0.0;
                        translation.y = 0.0;
                        commands.spawn(SpriteBundle {
                            sprite: Sprite { color, ..default() },
                            transform: Transform {
                                translation,
                                rotation: Quat::from_rotation_z(PI * 0.25),
                                scale: Vec3::new(15.0, 330.0 * SQRT_2, 1.0),
                            },
                            ..default()
                        });
                    }
                    Diagonal(_) => {
                        translation.x = 0.0;
                        translation.y = 0.0;
                        commands.spawn(SpriteBundle {
                            sprite: Sprite { color, ..default() },
                            transform: Transform {
                                translation,
                                rotation: Quat::from_rotation_z(PI * 0.75),
                                scale: Vec3::new(15.0, 330.0 * SQRT_2, 1.0),
                            },
                            ..default()
                        });
                    }
                    Tie => {
                        translation.x = 0.0;
                        translation.y = 0.0;
                        commands.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgb(0.4, 0.3, 0.6),
                                ..default()
                            },
                            transform: Transform {
                                translation,
                                rotation: Quat::from_rotation_z(PI * 0.0),
                                scale: Vec3::new(260.0, 170.0, 1.0),
                            },
                            ..default()
                        });
                        commands.spawn(
                            TextBundle::from_sections([TextSection::new(
                                "TIE",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 140.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            )])
                            .with_style(Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(170.0),
                                    left: Val::Px(320.0),
                                    ..default()
                                },
                                ..default()
                            }),
                        );
                    }
                    Incomplete => {
                        game_state.turn ^= true;
                    }
                }
                if game_state.end_game != Incomplete {
                    // Render Reset Button
                    translation.x = 320.0;
                    translation.y = -180.0;
                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.4, 0.3, 0.6),
                            ..default()
                        },
                        transform: Transform {
                            translation,
                            rotation: Quat::from_rotation_z(PI * 0.0),
                            scale: Vec3::new(120.0, 50.0, 1.0),
                        },
                        ..default()
                    });
                    commands.spawn(
                        TextBundle::from_sections([TextSection::new(
                            "RESET",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        )])
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            position: UiRect {
                                top: Val::Px(400.0),
                                left: Val::Px(675.0),
                                ..default()
                            },
                            ..default()
                        }),
                    );
                }
            }
        }
    }
}

fn close_button_system(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    touches: Res<Touches>,
) {
    let mut touched = false;
    for touch in touches.iter_just_pressed() {
        let position = touch.position();
        let translation = Vec3::new(position.x - 400.0, 240.0 - position.y, 2.0);

        if let Some(collision) = collide(
            Vec3::new(360.0, 200.0, 1.0),
            Vec2::new(40.0, 40.0),
            translation,
            Vec2::splat(1.0),
        ) {
            touched = true;
        }
    }
    if !touched {
        return;
    }
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }
        commands.entity(window).despawn();
    }
}

fn reset_button_system(
    mut commands: Commands,
    touches: Res<Touches>,
    entities: Query<
        Entity,
        (
            With<Transform>,
            Without<Grid>,
            Without<Target>,
            Without<CloseButton>,
        ),
    >,
    mut game_state: ResMut<GameState>,
) {
    let mut touched = false;
    for touch in touches.iter_just_pressed() {
        let position = touch.position();
        let translation = Vec3::new(position.x - 400.0, 240.0 - position.y, 2.0);

        if let Some(collision) = collide(
            Vec3::new(320.0, -180.0, 0.0),
            Vec2::new(120.0, 50.0),
            translation,
            Vec2::splat(1.0),
        ) {
            touched = true;
        }
    }
    if !touched {
        return;
    }
    // Action
    info!("just pressed reset button",);
    game_state.reset();
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_cross(commands: &mut Commands, mut translation: Vec3, scale: Vec3, color: Color) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: color,
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
            color: color,
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), Grid));

    // Rectangle.Vertical.Left
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(10.0, 350.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-60., 0., 0.)),
            ..default()
        },
        Grid,
    ));
    // Rectangle.Vertical.Right
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(10.0, 350.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(60., 0., 0.)),
            ..default()
        },
        Grid,
    ));
    // Rectangle.Horizontal.Top
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(350.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 60., 0.)),
            ..default()
        },
        Grid,
    ));
    // Rectangle.Horizontal.Bottom
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(350.0, 10.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -60., 0.)),
            ..default()
        },
        Grid,
    ));
    let mut index = 0;
    for y in (-120..=120).rev().step_by(120) {
        for x in (-120..=120).step_by(120) {
            commands.spawn((target(Vec3::new(x as f32, y as f32, 0.)), Target(index)));
            index += 1;
        }
    }
    // Close button
    let mut translation = Vec3::new(360.0, 200.0, 1.0);
    let scale = Vec3::new(10.0, 40.0, 1.0);
    let color = Color::rgb(0.6, 0.25, 0.25);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                translation,
                rotation: Quat::from_rotation_z(PI * 0.25),
                scale,
            },
            ..default()
        },
        CloseButton,
    ));
    translation.z = 3.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                translation,
                rotation: Quat::from_rotation_z(PI * 0.75),
                scale,
            },
            ..default()
        },
        CloseButton,
    ));
}

#[derive(Default, Eq, PartialEq)]
enum Cell {
    #[default]
    Empty,
    Nought,
    Cross,
}

#[derive(Default, Eq, PartialEq)]
enum EndGame {
    #[default]
    Incomplete,
    Tie,
    Row(usize),
    Column(usize),
    Diagonal(usize),
}

#[derive(Resource, Default)]
struct GameState {
    turn: bool,
    cells: [[Cell; 3]; 3],
    end_game: EndGame,
}

impl EndGame {
    fn new(cells: &[[Cell; 3]; 3]) -> Self {
        let complete = cells
            .iter()
            .find(|&row| row.iter().find(|&v| *v == Empty).is_some())
            .is_none();
        match cells {
            [[Nought, Nought, Nought], _, _] | [[Cross, Cross, Cross], _, _] => Row(0),
            [_, [Nought, Nought, Nought], _] | [_, [Cross, Cross, Cross], _] => Row(1),
            [_, _, [Nought, Nought, Nought]] | [_, _, [Cross, Cross, Cross]] => Row(2),
            [[Nought, _, _], [Nought, _, _], [Nought, _, _]]
            | [[Cross, _, _], [Cross, _, _], [Cross, _, _]] => Column(0),
            [[_, Nought, _], [_, Nought, _], [_, Nought, _]]
            | [[_, Cross, _], [_, Cross, _], [_, Cross, _]] => Column(1),
            [[_, _, Nought], [_, _, Nought], [_, _, Nought]]
            | [[_, _, Cross], [_, _, Cross], [_, _, Cross]] => Column(2),
            [[Nought, _, _], [_, Nought, _], [_, _, Nought]]
            | [[Cross, _, _], [_, Cross, _], [_, _, Cross]] => Diagonal(0),
            [[_, _, Nought], [_, Nought, _], [Nought, _, _]]
            | [[_, _, Cross], [_, Cross, _], [Cross, _, _]] => Diagonal(1),
            _ => {
                if complete {
                    Tie
                } else {
                    Incomplete
                }
            }
        }
    }
}

impl GameState {
    fn place(&mut self, cell: usize) -> bool {
        let (row, column) = (cell / 3, cell % 3);
        if self.cells[row][column] != Empty || self.end_game != Incomplete {
            return false;
        }
        self.cells[row][column] = if self.turn { Cross } else { Nought };
        self.end_game = EndGame::new(&self.cells);
        true
    }

    fn reset(&mut self) {
        *self = GameState::default();
    }
}

#[derive(Component)]
struct Grid;

#[derive(Component)]
struct CloseButton;

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
