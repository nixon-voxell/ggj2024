use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

// #[derive(Component)]
// pub struct Hovered;

#[derive(Component)]
pub struct Clicked;

#[derive(Component)]
pub struct Clickable;

pub fn mouse_hover(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_clickables: Query<With<Clickable>>,
    input_mouse: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
) {
    // Add Clicked component to hovered entity on clicked
    if input_mouse.just_pressed(MouseButton::Left) == false {
        return;
    }

    let Some(mut cursor_position) = q_windows.single().cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

    cursor_position = camera
        .viewport_to_world_2d(camera_transform, cursor_position)
        .unwrap();

    rapier_context.intersections_with_point(cursor_position, QueryFilter::default(), |entity| {
        if q_clickables.contains(entity) {
            commands.entity(entity).insert(Clicked);
            println!("Clicking entity {:?}", entity);
        }
        // true
        false
    });
}

pub fn clear_hover(
    mut commands: Commands,
    q_hovers: Query<Entity, With<Clicked>>,
    input_mouse: Res<Input<MouseButton>>,
) {
    // Remove Clicked component on clicked
    if input_mouse.just_pressed(MouseButton::Left) == false {
        return;
    }

    for entity in q_hovers.iter() {
        commands.entity(entity).remove::<Clicked>();
    }
}

pub fn hover_animation(
    mut q_clicks: Query<&mut Transform, With<Clicked>>,
    mut q_not_clicks: Query<&mut Transform, (Without<Clicked>, With<Clickable>)>,
) {
    for mut transform in q_clicks.iter_mut() {
        transform.scale = Vec3::splat(1.1);
    }

    for mut transform in q_not_clicks.iter_mut() {
        transform.scale = Vec3::splat(1.0);
    }
}
