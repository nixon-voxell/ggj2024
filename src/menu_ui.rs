use bevy::math::{DVec2, DVec4};
use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

#[derive(Component)]
pub struct StartBtn;

#[derive(Component)]
pub struct EndBtn;

/// Position in the board
#[derive(Component)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

pub fn menu_button(mut commands: Commands, mut fragments: ResMut<Assets<VelloFragment>>) {
    let palette: ColorPalette<ColorKey> = ColorPalette::default();
    let fill_color: Color = *palette.get_or_default(&ColorKey::Blue);
    let stroke_color: Color = fill_color * 1.5;
    let rect: VelloRectBundle = VelloRectBundle {
        rect: VelloRect::anchor_center(DVec2::new(200., 100.), DVec4::splat(0.0)),
        fill: FillStyle::from_brush(fill_color.with_a(0.0)),
        stroke: StrokeStyle::from_brush(stroke_color.with_a(1.0)).with_style(2.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            ..default()
        },
    };

    commands
        .spawn(rect.clone())
        .insert(StartBtn)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
    commands
        .spawn(rect)
        .insert(EndBtn)
        .insert(Transform::from_xyz(0.0, -120.0, 0.0));

    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.75),
    //             custom_size: Some(Vec2::new(200.0, 100.0)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    //         ..default()
    //     })
    //     .insert(StartBtn);

    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: Color::rgb(0.25, 0.25, 0.75),
    //             custom_size: Some(Vec2::new(200.0, 100.0)),
    //             ..default()
    //         },
    //         transform: Transform::from_translation(Vec3::new(0., -120., 0.)),
    //         ..default()
    //     })
    //     .insert(EndBtn);
}
