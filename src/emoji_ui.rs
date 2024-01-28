use bevy::{
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;
use motiongfx_typst::TypstCompiler;

use crate::{
    emoji::{self, EmojiMap},
    menu_ui, mouse, SetupTimeline,
};

#[derive(Component)]
pub struct TileSetupTimeline;

#[derive(Component)]
pub struct EmojiTile {
    pub index: usize,
}

#[derive(Component, Default)]
pub struct PlaySoundBtn;

#[derive(Component)]
pub struct PlayerSelection;

#[derive(Component)]
pub struct Menu;

const SPACING_SCALE: f32 = 3.0;
const STARTING_SCALE: Vec3 = Vec3::splat(0.5);
const LOTTIE_SCALE: Vec3 = Vec3::splat(0.05);

pub fn setup(mut commands: Commands, mut fragments: ResMut<Assets<VelloFragment>>) {
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

    let stroke_color: Color = *palette.get_or_default(&ColorKey::Base8);
    let mut tile_sequences: Vec<Sequence> = Vec::with_capacity(ROW_COUNT);

    for x in 0..ROW_COUNT {
        // Spawn board tiles
        let fill_color: Color = {
            if x % 2 == 0 {
                *palette.get_or_default(&ColorKey::Base0)
            } else {
                *palette.get_or_default(&ColorKey::Base6)
            }
        };

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
            ))
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
    mut emoji_map: ResMut<EmojiMap>,
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

    let emoji_keys: Vec<&String> = emoji_map.map.keys().collect();

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
            let entity: Entity = commands
                .spawn((
                    rect.clone(),
                    EmojiTile { index },
                    Collider::cuboid(HALF_TILE_SIZE, HALF_TILE_SIZE),
                    mouse::Clickable,
                ))
                .with_children(|parent| {
                    parent.spawn(bevy_vello::VelloVectorBundle {
                        vector: emoji_map.map[emoji_keys[index]].vector_handle.clone(),
                        transform: Transform::from_xyz(0.0, -TILE_SIZE * 0.5, 1.0)
                            .with_scale(LOTTIE_SCALE),
                        ..default()
                    });
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
    }

    let sequence: Sequence = flow(0.1, &tile_sequences).with_ease(ease::cubic::ease_in_out);
    let sequence_id: Entity = commands.spawn(sequence).id();

    let mut timeline: Timeline = Timeline::new(sequence_id);
    timeline.time_scale = -1.0;
    commands.spawn((timeline, SetupTimeline, TileSetupTimeline));
}

pub fn setup_play_sound_btn(
    mut commands: Commands,
    mut fragments: ResMut<Assets<VelloFragment>>,
    mut typst_compiler: ResMut<TypstCompiler>,
) {
    let palette: ColorPalette<ColorKey> = ColorPalette::default();
    let fill: Color = *palette.get_or_default(&ColorKey::Purple);

    let button_seq: Sequence = menu_ui::create_button::<PlaySoundBtn>(
        &mut commands,
        &mut fragments,
        &mut typst_compiler,
        DVec2::new(150.0, 100.0),
        100.0,
        fill,
        Vec3::new(500.0, -300.0, 0.0),
        Vec3::Y * 100.0,
        "= 🎵",
    );

    let sequence: Sequence = button_seq.with_ease(ease::cubic::ease_in_out);
    let sequence_id: Entity = commands.spawn(sequence).id();

    let mut timeline: Timeline = Timeline::new(sequence_id);
    timeline.time_scale = -1.0;
    commands.spawn((timeline, SetupTimeline, TileSetupTimeline));
}

pub fn play_sound_button_evt(
    q_play_sound_btns: Query<&PlaySoundBtn>,
    mut ev_clicked: EventReader<mouse::Clicked>,
    // mut ev_play_sound: EventWriter<emoji::PlaySound>,
) {
    for clicked in ev_clicked.read() {
        if let Ok(_) = q_play_sound_btns.get(clicked.entity) {
            // ev_play_sound.send();
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
