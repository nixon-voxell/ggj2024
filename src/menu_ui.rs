use bevy::math::{DVec2, DVec4};
use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::Collider;
use motiongfx_typst::TypstCompiler;
use motiongfx_vello::svg::SvgTreeBundle;

use crate::game::{GameState, GameStateRes};
use crate::{mouse, SetupTimeline};

#[derive(Component, Default)]
pub struct StartBtn;

#[derive(Component, Default)]
pub struct QuitBtn;

#[derive(Component)]
pub struct MenuSetupTimeline;

pub fn menu_button(
    mut commands: Commands,
    mut fragments: ResMut<Assets<VelloFragment>>,
    mut typst_compiler: ResMut<TypstCompiler>,
) {
    const OFFSET: Vec3 = Vec3::new(500.0, 0.0, 0.0);
    let palette: ColorPalette<ColorKey> = ColorPalette::default();
    let start_color: Color = *palette.get_or_default(&ColorKey::Blue);
    let quit_color: Color = *palette.get_or_default(&ColorKey::Red);

    let start_seq: Sequence = create_button::<StartBtn>(
        &mut commands,
        &mut fragments,
        &mut typst_compiler,
        DVec2::new(200.0, 100.0),
        100.0,
        start_color,
        Vec3::new(-500.0, 0.0, 0.0),
        OFFSET,
        "= Start",
    );

    let quit_seq: Sequence = create_button::<QuitBtn>(
        &mut commands,
        &mut fragments,
        &mut typst_compiler,
        DVec2::new(200.0, 100.0),
        100.0,
        quit_color,
        Vec3::new(-500.0, -200.0, 0.0),
        OFFSET,
        "= Quit",
    );

    let sequence: Sequence = flow(0.1, &[start_seq, quit_seq]).with_ease(ease::cubic::ease_in_out);
    let sequence_id: Entity = commands.spawn(sequence).id();

    let mut timeline: Timeline = Timeline::new(sequence_id);
    timeline.time_scale = 1.0;
    commands.spawn((timeline, SetupTimeline, MenuSetupTimeline));
}

pub fn start_button_evt(
    q_start_btns: Query<&StartBtn>,
    mut ev_clicked: EventReader<mouse::Clicked>,
    mut game_state: ResMut<GameStateRes>,
) {
    for clicked in ev_clicked.read() {
        if let Ok(_) = q_start_btns.get(clicked.entity) {
            game_state.target_state = GameState::InGame;
        }
    }
}

pub fn create_button<Comp: Component + Default>(
    commands: &mut Commands,
    fragments: &mut ResMut<Assets<VelloFragment>>,
    typst_compiler: &mut ResMut<TypstCompiler>,
    size: DVec2,
    radius: f64,
    fill: Color,
    translation: Vec3,
    translate_animation: Vec3,
    label: &str,
) -> Sequence {
    let rect: VelloRectBundle = create_rect(fragments, size, radius, fill.with_a(0.0), translation);

    let header: String = r###"
        #set page(width: 100pt, margin: 8pt)
        #set text(size: 24pt, font: "consolas", fill: rgb("#FCFCFA"))
    "###
    .into();

    let mut label_tree: SvgTreeBundle = typst_compiler
        .compile_flatten(commands, fragments, header + label)
        .unwrap();

    // move label content to the front
    commands
        .entity(label_tree.root_entity)
        .insert(Transform::from_xyz(-label_tree.size.x * 0.5, 0.0, 1.0));

    let path_len: usize = label_tree.paths.len();

    let mut fill_seqs: Vec<Sequence> = Vec::with_capacity(path_len);

    for path in label_tree.paths.iter_mut() {
        path.fill = Some(Color::BLACK.with_a(0.0).into());

        commands
            .entity(path.entity)
            .insert(path.fill.as_ref().unwrap().clone());

        let mut fill_motion: FillStyleMotion =
            FillStyleMotion::new(path.entity, path.fill.clone().unwrap());
        let mut transform_motion: TransformMotion =
            TransformMotion::new(path.entity, path.transform);

        let mut act: ActionBuilder = ActionBuilder::new(commands);
        fill_seqs.push(all(&[
            act.play(fill_motion.brush_to(Color::BLACK), 1.0),
            act.play(transform_motion.translate_add(Vec3::Y * 25.0), 1.0),
        ]));
    }

    let rect_id: Entity = commands
        .spawn((
            rect.clone(),
            Comp::default(),
            Collider::cuboid(size.x as f32 * 0.5, size.y as f32 * 0.5),
            mouse::Clickable,
        ))
        .push_children(&[label_tree.root_entity])
        .id();

    let mut rect_motion: VelloRectBundleMotion = VelloRectBundleMotion::new(rect_id, rect);

    let mut act: ActionBuilder = ActionBuilder::new(commands);
    flow(
        0.1,
        &[
            all(&[
                act.play(
                    rect_motion.transform.translate_add(translate_animation),
                    1.0,
                ),
                act.play(rect_motion.fill.brush_to(fill), 1.0),
            ]),
            flow(0.1, &fill_seqs),
        ],
    )
}

fn create_rect(
    fragments: &mut ResMut<Assets<VelloFragment>>,
    size: DVec2,
    radius: f64,
    fill: Color,
    translation: Vec3,
) -> VelloRectBundle {
    VelloRectBundle {
        rect: VelloRect::anchor_center(size, DVec4::splat(radius)),
        fill: FillStyle::from_brush(fill),
        stroke: StrokeStyle::from_brush(Color::NONE).with_style(0.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_translation(translation)),
            ..default()
        },
    }
}
