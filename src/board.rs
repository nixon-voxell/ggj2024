use bevy::math::{DVec2, DVec4};
use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

#[derive(Component)]
pub struct RedRook;

#[derive(Component)]
pub struct BlueRook;

/// Position in the board
#[derive(Component)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct RookAlive;

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Copy, Default)]
pub enum TileState {
    #[default]
    Empty,
    Blue(Option<Entity>),
    Red(Option<Entity>),
}

#[derive(Resource, Clone)]
pub struct Board {
    pub tiles: Vec<TileState>,
    /// Number of rows.
    pub row_count: usize,
    /// Size of each tile.
    pub tile_size: f32,
}

impl Board {
    pub fn new(row_count: usize, tile_size: f32) -> Self {
        let mut tiles = Vec::new();
        tiles.resize_with(row_count * row_count, || TileState::Empty);
        Self {
            tiles,
            row_count,
            tile_size,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileState {
        self.tiles[self.get_tile_index(x, y)]
    }

    pub fn get_tile_index(&self, x: usize, y: usize) -> usize {
        x + y * self.row_count
    }
}

#[derive(Component)]
pub struct BoardSetupTimeline;

pub fn setup(mut commands: Commands, mut fragments: ResMut<Assets<VelloFragment>>) {
    // Number of rows on the board
    const ROW_COUNT: usize = 6;
    // Total nuber of tiles
    const TILE_COUNT: usize = ROW_COUNT * ROW_COUNT;

    // The size of a single tile
    const TILE_SIZE: f32 = 100.0;
    const ROOK_RADIUS: f64 = 30.0;
    const HALF_TILE_SIZE: f32 = TILE_SIZE * 0.5;

    // The position where the tile should start at (both x and y axes)
    const ROW_START: f32 = -(ROW_COUNT as f32 * TILE_SIZE) * 0.5 + HALF_TILE_SIZE;
    const SPACING_SCALE: f32 = 1.5;
    const STARTING_SCALE: Vec3 = Vec3::splat(0.5);

    // Color palette
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    let mut tile_sequences: Vec<Sequence> = Vec::with_capacity(TILE_COUNT);
    let mut rook_sequences: Vec<Sequence> = Vec::with_capacity(ROW_COUNT * 2);

    let stroke_color: Color = *palette.get_or_default(&ColorKey::Base8);

    for x in 0..ROW_COUNT {
        for y in 0..ROW_COUNT {
            // Spawn board tiles
            let fill_color: Color = {
                if x % 2 == y % 2 {
                    *palette.get_or_default(&ColorKey::Base0)
                } else {
                    *palette.get_or_default(&ColorKey::Base6)
                }
            };

            let translation: Vec3 = Vec3::new(
                TILE_SIZE * (x as f32) + ROW_START,
                TILE_SIZE * (y as f32) + ROW_START,
                0.0,
            );

            let rect: VelloRectBundle = VelloRectBundle {
                rect: VelloRect::anchor_center(DVec2::splat(TILE_SIZE as f64), DVec4::splat(0.0)),
                fill: FillStyle::from_brush(fill_color.with_a(0.0)),
                stroke: StrokeStyle::from_brush(stroke_color.with_a(0.0)).with_style(2.0),
                fragment_bundle: VelloFragmentBundle {
                    fragment: fragments.add(VelloFragment::default()),
                    transform: TransformBundle::from_transform(
                        Transform::from_translation(translation * SPACING_SCALE)
                            .with_scale(STARTING_SCALE),
                    ),
                    ..default()
                },
            };

            let entity: Entity = commands
                .spawn((rect.clone(), Tile, BoardPosition { x, y }))
                .id();

            let mut rect_motion: VelloRectBundleMotion = VelloRectBundleMotion::new(entity, rect);

            // Board tile initial animation
            let mut act: ActionBuilder = ActionBuilder::new(&mut commands);
            tile_sequences.push(all(&[
                act.play(rect_motion.transform.translate_to(translation), 0.6),
                act.play(rect_motion.transform.scale_to(Vec3::splat(1.0)), 0.6),
                act.play(rect_motion.fill.brush_to(fill_color), 0.6),
                act.play(rect_motion.stroke.brush_to(stroke_color), 0.6),
            ]));
        }
    }

    // Initalize board resource
    let mut board: Board = Board::new(ROW_COUNT, TILE_SIZE);

    // First row rooks (blue)
    let fill_color: Color = *palette.get_or_default(&ColorKey::Blue);
    let stroke_color: Color = fill_color * 1.5;

    for x in 0..ROW_COUNT {
        let translation: Vec3 = Vec3::new(TILE_SIZE * (x as f32) + ROW_START, ROW_START, 0.0);

        let circle: VelloCircleBundle = VelloCircleBundle {
            circle: VelloCircle::from_radius(ROOK_RADIUS),
            fill: FillStyle::from_brush(fill_color.with_a(0.0)),
            stroke: StrokeStyle::from_brush(stroke_color.with_a(0.0)).with_style(6.0),
            fragment_bundle: VelloFragmentBundle {
                fragment: fragments.add(VelloFragment::default()),
                transform: TransformBundle::from_transform(
                    Transform::from_translation(translation).with_scale(STARTING_SCALE),
                ),
                ..default()
            },
        };

        let circle_id: Entity = commands
            .spawn((
                circle.clone(),
                BlueRook,
                BoardPosition { x, y: 0 },
                RookAlive,
            ))
            .id();

        // Add rook to board
        board.tiles[x] = TileState::Blue(Some(circle_id));

        let mut circle_motion: VelloCircleBundleMotion =
            VelloCircleBundleMotion::new(circle_id, circle);

        let mut act: ActionBuilder = ActionBuilder::new(&mut commands);
        rook_sequences.push(all(&[
            act.play(circle_motion.transform.scale_to(Vec3::splat(1.0)), 0.6),
            act.play(circle_motion.fill.brush_to(fill_color), 0.6),
            act.play(circle_motion.stroke.brush_to(stroke_color), 0.6),
        ]));
    }

    // Last row rooks (red)
    let fill_color: Color = *palette.get_or_default(&ColorKey::Red);
    let stroke_color: Color = fill_color * 1.5;

    for x in 0..ROW_COUNT {
        let translation: Vec3 = Vec3::new(
            TILE_SIZE * (x as f32) + ROW_START,
            TILE_SIZE * (ROW_COUNT as f32 - 1.0) + ROW_START,
            0.0,
        );

        let circle: VelloCircleBundle = VelloCircleBundle {
            circle: VelloCircle::from_radius(ROOK_RADIUS),
            fill: FillStyle::from_brush(fill_color.with_a(0.0)),
            stroke: StrokeStyle::from_brush(stroke_color.with_a(0.0)).with_style(6.0),
            fragment_bundle: VelloFragmentBundle {
                fragment: fragments.add(VelloFragment::default()),
                transform: TransformBundle::from_transform(
                    Transform::from_translation(translation).with_scale(STARTING_SCALE),
                ),
                ..default()
            },
        };

        let circle_id: Entity = commands
            .spawn((
                circle.clone(),
                RedRook,
                BoardPosition {
                    x,
                    y: ROW_COUNT - 1,
                },
                RookAlive,
            ))
            .id();

        // Add rook to board
        board.tiles[TILE_COUNT - ROW_COUNT + x] = TileState::Red(Some(circle_id));

        let mut circle_motion: VelloCircleBundleMotion =
            VelloCircleBundleMotion::new(circle_id, circle);

        let mut act: ActionBuilder = ActionBuilder::new(&mut commands);
        rook_sequences.push(all(&[
            act.play(circle_motion.transform.scale_to(Vec3::splat(1.0)), 0.6),
            act.play(circle_motion.fill.brush_to(fill_color), 0.6),
            act.play(circle_motion.stroke.brush_to(stroke_color), 0.6),
        ]));
    }

    let sequence: Sequence = chain(&[flow(0.04, &tile_sequences), flow(0.04, &rook_sequences)]);
    let sequence_id: Entity = commands.spawn(sequence).id();
    commands.spawn((Timeline::new(sequence_id), BoardSetupTimeline));

    commands.insert_resource(board);
}

pub fn setup_animation_update(
    mut q_timelines: Query<&mut Timeline, &BoardSetupTimeline>,
    q_sequences: Query<&Sequence>,
    time: Res<Time>,
) {
    let Ok(mut timeline) = q_timelines.get_single_mut() else {
        return;
    };

    let Ok(sequence) = q_sequences.get(timeline.sequence_id().unwrap()) else {
        return;
    };

    // stops updating when timeline reaches the end
    if (timeline.time_scale > 0.0 && timeline.target_time >= sequence.duration())
        || (timeline.time_scale < 0.0 && timeline.target_time <= 0.0)
    {
        return;
    }

    timeline.target_time += timeline.time_scale * time.delta_seconds();
}
