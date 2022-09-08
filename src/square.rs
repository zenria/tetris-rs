use bevy::{
    prelude::{shape::Quad, *},
    sprite::MaterialMesh2dBundle,
};

use crate::{
    board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH},
    piece::Piece,
};

#[derive(Component, Clone, Copy)]
pub struct Square;

pub const SQ_SIZE: f32 = 20.;

pub const SQ_BORDER_WIDTH: f32 = 2.;

/// Marker component: walls&floor
#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
struct SquareBundle {
    square: Square,
    board_position: BoardPosition,
    #[bundle]
    spatial_bundle: SpatialBundle,
}

pub const SQ_TOTAL_SIZE: f32 = SQ_SIZE + SQ_BORDER_WIDTH;
const BOARD_LEFT_X: f32 = -(BOARD_WIDTH as f32) / 2. * SQ_TOTAL_SIZE;
const BOARD_BOTTOM_Y: f32 = -(BOARD_HEIGHT as f32) / 2. * SQ_TOTAL_SIZE;

impl SquareBundle {
    fn from_board_position(square: Square, board_position: BoardPosition) -> Self {
        Self {
            square,
            spatial_bundle: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    BOARD_LEFT_X + SQ_TOTAL_SIZE * (board_position.x as f32),
                    BOARD_BOTTOM_Y + SQ_TOTAL_SIZE * (board_position.y as f32),
                    0.,
                )),
                ..Default::default()
            },
            board_position,
        }
    }
}

pub fn spawn_square<T:Component>(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    board_position: BoardPosition,
    color: Color,
    square: Square,
    adjacent_componnmt: Option<T>,
) -> Entity {
    let mut entity =
        commands.spawn_bundle(SquareBundle::from_board_position(square, board_position));

    entity.with_children(|commands| {
        let mut bgcolor = color.clone();
        bgcolor.set_a(0.25);
        // background
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Quad {
                        size: Vec2::new(SQ_SIZE, SQ_SIZE),
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(bgcolor)),

            ..default()
        });
        // left
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Quad {
                        size: Vec2::new(SQ_BORDER_WIDTH, SQ_SIZE),
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(
                -SQ_SIZE / 2. + SQ_BORDER_WIDTH / 2.,
                0.,
                0.,
            )),
            ..default()
        });
        // right
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Quad {
                        size: Vec2::new(SQ_BORDER_WIDTH, SQ_SIZE),
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(
                SQ_SIZE / 2. - SQ_BORDER_WIDTH / 2.,
                0.,
                0.,
            )),
            ..default()
        });
        // bottom
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Quad {
                        size: Vec2::new(SQ_SIZE, SQ_BORDER_WIDTH),
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(
                0.,
                -SQ_SIZE / 2. + SQ_BORDER_WIDTH / 2.,
                0.,
            )),
            ..default()
        });
        // top
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    Quad {
                        size: Vec2::new(SQ_SIZE, SQ_BORDER_WIDTH),
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(
                0.,
                SQ_SIZE / 2. - SQ_BORDER_WIDTH / 2.,
                0.,
            )),
            ..default()
        });
    });
    if let Some(c) = adjacent_componnmt {
        entity.insert(c);
    }
    entity.id()
}
