use bevy::prelude::*;

#[derive(Component)]
pub struct StartBtn;

#[derive(Component)]
pub struct EndBtn;

pub fn menu_button(mut commands: Commands) {
    // let rect: VelloRectBundle = VelloRectBundle {
    //     rect: VelloRect::anchor_center(DVec2::splat(TILE_SIZE as f64), DVec4::splat(0.0)),
    //     fill: FillStyle::from_brush(fill_color.with_a(0.0)),
    //     stroke: StrokeStyle::from_brush(stroke_color.with_a(0.0)).with_style(2.0),
    //     fragment_bundle: VelloFragmentBundle {
    //         fragment: fragments.add(VelloFragment::default()),
    //         transform: TransformBundle::from_transform(
    //             Transform::from_translation(translation * SPACING_SCALE).with_scale(STARTING_SCALE),
    //         ),
    //         ..default()
    //     },
    // };
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(200.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(StartBtn);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(200.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -120., 0.)),
            ..default()
        })
        .insert(EndBtn);
}
