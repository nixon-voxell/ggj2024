use bevy::math::{DVec2, DVec4};
use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .add_systems(Startup, (setup, spawn_bot, spawn_player))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Directional light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_bot(mut commands: Commands, mut fragments: ResMut<Assets<VelloFragment>>) {
    const ROW_COUNT: u32 = 6;
    const COL_COUNT: u32 = 6;
    const TILE_SIZE: f32 = 100.0;
    const HALF_TILE_SIZE: f32 = TILE_SIZE * 0.5;
    let left_end_x: f32 = -(ROW_COUNT as f32 * TILE_SIZE) * 0.5 + HALF_TILE_SIZE;
    let left_end_y: f32 = -(ROW_COUNT as f32 * TILE_SIZE) * 0.5 + HALF_TILE_SIZE;

    // Color palette
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    // Spawning entities
    let rect_bundle: VelloRectBundle = VelloRectBundle {
        rect: VelloRect::anchor_center(DVec2::new(100.0, 100.0), DVec4::splat(10.0)),
        fill: FillStyle::from_brush(*palette.get_or_default(&ColorKey::Blue)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Blue) * 1.5)
            .with_style(4.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_xyz(-200.0, 0.0, 0.0)),
            ..default()
        },
    };

    for x in 0..ROW_COUNT {
        for y in 0..COL_COUNT {
            commands
                .spawn(rect_bundle.clone())
                .insert(Transform::from_xyz(
                    TILE_SIZE * (x as f32) + left_end_x,
                    TILE_SIZE * (y as f32) + left_end_y,
                    0.0,
                ));
        }
    }
}

fn spawn_player(mut commands: Commands, mut fragments: ResMut<Assets<VelloFragment>>) {
    // Color palette
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    // Spawning entities
    let circ_bundle: VelloCircleBundle = VelloCircleBundle {
        circle: VelloCircle::from_radius(50.0),
        fill: FillStyle::from_brush(*palette.get_or_default(&ColorKey::Purple)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Purple) * 1.5)
            .with_style(4.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_xyz(200.0, 0.0, 0.0)),
            ..default()
        },
    };

    // commands.spawn(circ_bundle);
}
