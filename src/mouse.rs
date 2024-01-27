use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Hovered;

pub fn mouse_hover(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
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
        // Callback called on each collider with a shape containing the point.
        println!("The entity {:?} contains the point.", entity);
        // Return `false` instead if we want to stop searching for other colliders containing this point.
        true
    });
}

pub fn clear_hover(mut commands: Commands, q_hovers: Query<Entity, With<Hovered>>) {
    for entity in q_hovers.iter() {
        commands.entity(entity).remove::<Hovered>();
    }
}

pub fn hover_animation(
    mut q_hovers: Query<&mut Transform, With<Hovered>>,
    // mut q_non_hovers: Query<&mut Transform, (Without<Hovered>, With<Hoverable>)>,
) {
    for mut transform in q_hovers.iter_mut() {
        transform.scale = Vec3::splat(1.1);
    }

    // for mut transform in q_non_hovers.iter_mut() {
    //     transform.scale = Vec3::splat(1.0);
    // }
}
