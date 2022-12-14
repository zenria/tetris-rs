use bevy::{
    prelude::{shape::Quad, *},
    sprite::MaterialMesh2dBundle,
};

use crate::{
    board::{BoardPosition, BOARD_HEIGHT, BOARD_WIDTH},
    SpawnPieceEvent,
};

/// Each item on the board is a Square: pieces are composed
/// with squares, walls and floor are made with squares.
///
/// When a piece hit the floor or other (immibilized) pieces
/// while moving down, squares are left on the board (they
/// are not despawned, only the Piece component is removed)
///
/// Squares are themselves composed with:
/// - a background Quad
/// - 4 thin Quads to form borders

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
const BOARD_LEFT_X: f32 = -((BOARD_WIDTH as f32) + 9.) / 2. * SQ_TOTAL_SIZE;
const BOARD_BOTTOM_Y: f32 = -(BOARD_HEIGHT as f32) / 2. * SQ_TOTAL_SIZE;

impl BoardPosition {
    pub fn to_real_position(&self) -> Transform {
        Transform::from_translation(Vec3::new(
            BOARD_LEFT_X + SQ_TOTAL_SIZE * (self.x as f32),
            BOARD_BOTTOM_Y + SQ_TOTAL_SIZE * (self.y as f32),
            0.,
        ))
    }
}

impl SquareBundle {
    fn from_board_position(square: Square, board_position: BoardPosition) -> Self {
        Self {
            square,
            spatial_bundle: SpatialBundle {
                transform: board_position.to_real_position(),
                ..Default::default()
            },
            board_position,
        }
    }
}

pub fn spawn_square<T: Component>(
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

#[derive(Component, Default)]
pub struct DisappearingSquare {
    completion: f32,
}

const DISAPEARING_VELOCITY: f32 = 5.;

pub fn disappearing_square(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut sq_query: Query<(Entity, &mut DisappearingSquare, &Children)>,
    mut query: Query<&mut Handle<ColorMaterial>>,
    mut spawn_next_piece: EventWriter<SpawnPieceEvent>,
    mut move_below: EventWriter<MoveBelowEvent>,
) {
    let delta = time.delta().as_secs_f32();

    let mut ended = false;

    for (square_entity, mut disappearing, square_children) in &mut sq_query {
        disappearing.completion += DISAPEARING_VELOCITY * delta;
        if disappearing.completion >= 1. {
            ended = true;
            commands.entity(square_entity).despawn_recursive();
        } else {
            for square_child in square_children {
                // all children of a square have a color...
                let cm = query.get_mut(*square_child).unwrap();
                let color = &mut materials.get_mut(&cm).unwrap().color;
                let new_alpha = color.a() - DISAPEARING_VELOCITY * delta;
                if new_alpha >= 0. {
                    color.set_a(new_alpha);
                }
            }
        }
    }

    if ended {
        move_below.send_default();
        spawn_next_piece.send_default();
    }
}

///Square that must be moved below after new lines as been completed
#[derive(Component)]
pub struct ToMoveBelow(pub i32);

#[derive(Default)]
pub struct MoveBelowEvent;

pub fn to_move_below(
    mut commands: Commands,
    mut move_below: EventReader<MoveBelowEvent>,
    mut query: Query<(Entity, &ToMoveBelow, &mut BoardPosition, &mut Transform)>,
) {
    for _ in move_below.iter() {
        // move down
        for (e, num_of_lines, mut bp, mut tr) in &mut query {
            bp.y -= num_of_lines.0;
            tr.translation.y -= num_of_lines.0 as f32 * SQ_TOTAL_SIZE;
            commands.entity(e).remove::<ToMoveBelow>();
        }
    }
}
