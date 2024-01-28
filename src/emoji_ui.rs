use bevy::{
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_vello::{VelloVector, VelloVectorBundle};
use motiongfx_typst::TypstCompiler;

use crate::{
    emoji::{self, EmojiMap, RandomNumber},
    game, menu_ui, mouse, SetupTimeline,
};

#[derive(Resource)]
pub struct EmojiGuesses {
    pub placement_tiles: [Option<Entity>; 4],
    pub answer_tiles: [Option<Entity>; 4],
    pub numbers: [i32; 4],
}

impl Default for EmojiGuesses {
    fn default() -> Self {
        Self {
            placement_tiles: [None; 4],
            answer_tiles: [None; 4],
            numbers: [-1, -1, -1, -1],
        }
    }
}

#[derive(Component)]
pub struct TileSetupTimeline;

#[derive(Component)]
pub struct EmojiTile {
    pub index: usize,
}

#[derive(Component)]
pub struct EmojiMenuBtn;

#[derive(Component)]
pub struct PlacementMenuBtn;

#[derive(Resource)]
pub struct PlacementIndex(pub usize);

#[derive(Component, Default)]
pub struct PlaySoundBtn;

#[derive(Component, Default)]
pub struct NextBtn;

#[derive(Component, Default)]
pub struct ExitBtn;

#[derive(Component)]
pub struct PlayerSelection;

#[derive(Component)]
pub struct Menu;

const SPACING_SCALE: f32 = 3.0;
const STARTING_SCALE: Vec3 = Vec3::splat(0.5);
const LOTTIE_SCALE: Vec3 = Vec3::splat(0.05);

pub fn setup(
    mut commands: Commands,
    mut fragments: ResMut<Assets<VelloFragment>>,
    mut emoji_guesses: ResMut<EmojiGuesses>,
) {
    // Number of rows on the board
    const ROW_COUNT: usize = 4;
    // The size of a single tile
    const TILE_SIZE: f32 = 200.0;
    const HALF_TILE_SIZE: f32 = TILE_SIZE * 0.5;

    // The position where the tile should start at (both x and y axes)
    const ROW_START: f32 = -(ROW_COUNT as f32 * TILE_SIZE) * 0.5 + HALF_TILE_SIZE;
    const OFFSET: Vec3 = Vec3::new(0.0, 200.0, 0.0);

    // Color palette
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    let fill_color: Color = *palette.get_or_default(&ColorKey::Base0);
    let stroke_color: Color = *palette.get_or_default(&ColorKey::Base8);
    let mut tile_sequences: Vec<Sequence> = Vec::with_capacity(ROW_COUNT);

    for x in 0..ROW_COUNT {
        // Spawn placement tiles
        let translation: Vec3 = Vec3::new(TILE_SIZE * (x as f32) + ROW_START, 0.0, -1.0);

        let rect: VelloRectBundle = create_tile(
            &mut fragments,
            TILE_SIZE as f64,
            fill_color,
            stroke_color,
            translation,
            OFFSET,
            SPACING_SCALE,
            STARTING_SCALE,
        );

        let entity: Entity = commands
            .spawn((
                rect.clone(),
                EmojiTile { index: x },
                Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
                mouse::Clickable,
                PlacementMenuBtn,
            ))
            .with_children(|parent| {
                emoji_guesses.placement_tiles[x] = Some(parent.spawn_empty().id());
                emoji_guesses.answer_tiles[x] = Some(parent.spawn_empty().id());
            })
            .id();

        let mut rect_motion: VelloRectBundleMotion = VelloRectBundleMotion::new(entity, rect);

        // Tile initial animation
        tile_sequences.push(create_tile_animation(
            &mut commands,
            &mut rect_motion,
            translation + OFFSET,
            fill_color,
            stroke_color,
        ));
    }

    let sequence: Sequence = flow(0.1, &tile_sequences).with_ease(ease::cubic::ease_in_out);
    let sequence_id: Entity = commands.spawn(sequence).id();

    let mut timeline: Timeline = Timeline::new(sequence_id);
    timeline.time_scale = -1.0;
    commands.spawn((timeline, SetupTimeline, TileSetupTimeline));
}

pub fn setup_menu(
    mut commands: Commands,
    mut fragments: ResMut<Assets<VelloFragment>>,
    emoji_map: Res<EmojiMap>,
) {
    // Number of rows on the board
    const ROW_COUNT: usize = 5;
    // The size of a single tile
    const TILE_SIZE: f32 = 50.0;
    const HALF_TILE_SIZE: f32 = TILE_SIZE * 0.5;

    // The position where the tile should start at (both x and y axes)
    const ROW_START: f32 = -(ROW_COUNT as f32 * TILE_SIZE) * 0.5 + HALF_TILE_SIZE;
    const OFFSET: Vec3 = Vec3::new(0.0, -100.0, 0.0);

    // Color palette
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    let stroke_color: Color = *palette.get_or_default(&ColorKey::Base8);
    let fill_color: Color = *palette.get_or_default(&ColorKey::Base2);
    let mut tile_sequences: Vec<Sequence> = Vec::with_capacity(ROW_COUNT);

    for x in 0..ROW_COUNT {
        for y in 0..ROW_COUNT {
            // Spawn board tiles
            let translation: Vec3 = Vec3::new(
                TILE_SIZE * (x as f32) + ROW_START,
                TILE_SIZE * (y as f32) + ROW_START,
                -1.0,
            );

            let rect: VelloRectBundle = create_tile(
                &mut fragments,
                TILE_SIZE as f64,
                fill_color,
                stroke_color,
                translation,
                OFFSET,
                SPACING_SCALE,
                STARTING_SCALE,
            );

            let index: usize = x + y * ROW_COUNT;
            let mut icon_id: Option<Entity> = None;
            let entity: Entity = commands
                .spawn((
                    rect.clone(),
                    EmojiTile { index },
                    Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
                    mouse::Clickable,
                    EmojiMenuBtn,
                ))
                .with_children(|parent| {
                    icon_id = Some(
                        parent
                            .spawn(bevy_vello::VelloVectorBundle {
                                vector: emoji_map.data[index].vector_handle.clone(),
                                transform: Transform::from_xyz(0.0, -TILE_SIZE * 0.5, 1.0)
                                    .with_scale(Vec3::splat(0.0)),
                                ..default()
                            })
                            .id(),
                    );
                })
                .id();

            let mut rect_motion: VelloRectBundleMotion = VelloRectBundleMotion::new(entity, rect);
            let mut icon_motion: TransformMotion = TransformMotion::new(
                icon_id.unwrap(),
                Transform::from_xyz(0.0, -TILE_SIZE * 0.5, 1.0).with_scale(Vec3::splat(0.0)),
            );

            // Tile initial animation
            tile_sequences.push(create_tile_animation(
                &mut commands,
                &mut rect_motion,
                translation + OFFSET,
                fill_color,
                stroke_color,
            ));
            let mut act: ActionBuilder = ActionBuilder::new(&mut commands);
            tile_sequences.push(act.play(icon_motion.scale_to(LOTTIE_SCALE), 1.0));
        }
    }

    let sequence: Sequence = flow(0.02, &tile_sequences).with_ease(ease::cubic::ease_in_out);
    let sequence_id: Entity = commands.spawn(sequence).id();

    let mut timeline: Timeline = Timeline::new(sequence_id);
    timeline.time_scale = -1.0;
    commands.spawn((timeline, SetupTimeline, TileSetupTimeline));
}

pub fn setup_action_btn(
    mut commands: Commands,
    mut fragments: ResMut<Assets<VelloFragment>>,
    mut typst_compiler: ResMut<TypstCompiler>,
) {
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    let play_btn_seq: Sequence = menu_ui::create_button::<PlaySoundBtn>(
        &mut commands,
        &mut fragments,
        &mut typst_compiler,
        DVec2::new(200.0, 70.0),
        100.0,
        *palette.get_or_default(&ColorKey::Purple),
        Vec3::new(500.0, -100.0, 0.0),
        Vec3::Y * 100.0,
        "= \\~ 🎵",
    );

    let next_btn_seq: Sequence = menu_ui::create_button::<NextBtn>(
        &mut commands,
        &mut fragments,
        &mut typst_compiler,
        DVec2::new(200.0, 70.0),
        100.0,
        *palette.get_or_default(&ColorKey::Orange),
        Vec3::new(500.0, -200.0, 0.0),
        Vec3::Y * 100.0,
        "= Next",
    );

    let exit_btn_seq: Sequence = menu_ui::create_button::<ExitBtn>(
        &mut commands,
        &mut fragments,
        &mut typst_compiler,
        DVec2::new(200.0, 70.0),
        100.0,
        *palette.get_or_default(&ColorKey::Red),
        Vec3::new(500.0, -300.0, 0.0),
        Vec3::Y * 100.0,
        "= Exit",
    );

    let sequence: Sequence =
        flow(0.1, &[play_btn_seq, next_btn_seq, exit_btn_seq]).with_ease(ease::cubic::ease_in_out);
    let sequence_id: Entity = commands.spawn(sequence).id();

    let mut timeline: Timeline = Timeline::new(sequence_id);
    timeline.time_scale = -1.0;
    commands.spawn((timeline, SetupTimeline, TileSetupTimeline));
}

pub fn play_sound_button_evt(
    q_play_sound_btns: Query<&PlaySoundBtn>,
    mut ev_clicked: EventReader<mouse::Clicked>,
    mut ev_play_sound: EventWriter<emoji::PlaySound>,
) {
    for clicked in ev_clicked.read() {
        if let Ok(_) = q_play_sound_btns.get(clicked.entity) {
            ev_play_sound.send(emoji::PlaySound);
        }
    }
}

pub fn placement_tiles_evt(
    q_placement_tile: Query<&EmojiTile, With<PlacementMenuBtn>>,
    mut ev_clicked: EventReader<mouse::Clicked>,
    mut placement_index: ResMut<PlacementIndex>,
) {
    for clicked in ev_clicked.read() {
        if let Ok(placement_tile) = q_placement_tile.get(clicked.entity) {
            placement_index.0 = placement_tile.index;
        }
    }
}

pub fn emoji_tiles_evt(
    mut commands: Commands,
    q_emoji_tiles: Query<&EmojiTile, With<EmojiMenuBtn>>,
    mut ev_clicked: EventReader<mouse::Clicked>,
    mut guesses: ResMut<EmojiGuesses>,
    emoji_map: Res<EmojiMap>,
    random_number: Res<RandomNumber>,
) {
    for clicked in ev_clicked.read() {
        if let Ok(emoji_tile) = q_emoji_tiles.get(clicked.entity) {
            if array_contain_number(&guesses.numbers, emoji_tile.index as i32) {
                continue;
            }

            for n in 0..guesses.numbers.len() {
                if guesses.numbers[n] == -1 {
                    println!("guess index: {}", emoji_tile.index);
                    guesses.numbers[n] = emoji_tile.index as i32;

                    commands.entity(guesses.placement_tiles[n].unwrap()).insert(
                        bevy_vello::VelloVectorBundle {
                            vector: emoji_map.data[emoji_tile.index].vector_handle.clone(),
                            transform: Transform::from_xyz(0.0, -50.0, 1.0)
                                .with_scale(Vec3::splat(0.1)),
                            ..default()
                        },
                    );

                    // last guess
                    if n == 3 {
                        for t in 0..guesses.placement_tiles.len() {
                            commands.entity(guesses.answer_tiles[t].unwrap()).insert(
                                bevy_vello::VelloVectorBundle {
                                    vector: emoji_map.data[random_number.numbers[t]]
                                        .vector_handle
                                        .clone(),
                                    transform: Transform::from_xyz(0.0, 20.0, 1.0)
                                        .with_scale(Vec3::splat(0.08)),
                                    ..default()
                                },
                            );
                        }
                    }
                    break;
                }
            }
        }
    }
}

pub fn array_contain_number<T: Eq + PartialEq>(array: &[T], number: T) -> bool {
    for a in array {
        if *a == number {
            return true;
        }
    }
    false
}

pub fn next_btn_evt(
    mut commands: Commands,
    q_next_btn: Query<With<NextBtn>>,
    mut ev_clicked: EventReader<mouse::Clicked>,
    mut ev_gen_rand_num: EventWriter<emoji::GenerateRandomNumber>,
    mut guesses: ResMut<EmojiGuesses>,
) {
    for clicked in ev_clicked.read() {
        if let Ok(_) = q_next_btn.get(clicked.entity) {
            for t in 0..guesses.placement_tiles.len() {
                guesses.numbers[t] = -1;

                commands
                    .entity(guesses.placement_tiles[t].unwrap())
                    .remove::<Handle<bevy_vello::VelloVector>>();

                commands
                    .entity(guesses.answer_tiles[t].unwrap())
                    .remove::<Handle<bevy_vello::VelloVector>>();
            }

            ev_gen_rand_num.send(emoji::GenerateRandomNumber);
        }
    }
}

pub fn exit_btn_evt(
    q_exit_btn: Query<With<ExitBtn>>,
    mut ev_clicked: EventReader<mouse::Clicked>,
    mut game_state: ResMut<game::GameStateRes>,
) {
    for clicked in ev_clicked.read() {
        if let Ok(_) = q_exit_btn.get(clicked.entity) {
            game_state.target_state = game::GameState::Start;
        }
    }
}

fn create_tile(
    fragments: &mut ResMut<Assets<VelloFragment>>,
    tile_size: f64,
    fill: Color,
    stroke: Color,
    translation: Vec3,
    offset: Vec3,
    spacing_scale: f32,
    starting_scale: Vec3,
) -> VelloRectBundle {
    VelloRectBundle {
        rect: VelloRect::anchor_center(DVec2::splat(tile_size), DVec4::splat(0.0)),
        fill: FillStyle::from_brush(fill.with_a(0.0)),
        stroke: StrokeStyle::from_brush(stroke.with_a(0.0)).with_style(2.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(
                Transform::from_translation((translation * spacing_scale) + offset)
                    .with_scale(starting_scale),
            ),
            ..default()
        },
    }
}

fn create_tile_animation(
    commands: &mut Commands,
    rect_motion: &mut VelloRectBundleMotion,
    translation: Vec3,
    fill: Color,
    stroke: Color,
) -> Sequence {
    let mut act: ActionBuilder = ActionBuilder::new(commands);
    all(&[
        act.play(rect_motion.transform.translate_to(translation), 1.0),
        act.play(rect_motion.transform.scale_to(Vec3::splat(1.0)), 1.0),
        act.play(rect_motion.fill.brush_to(fill), 1.0),
        act.play(rect_motion.stroke.brush_to(stroke), 1.0),
    ])
}
