use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

#[derive(Event)]
pub struct Clicked {
    pub entity: Entity,
    pub prev_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct PreviousClicked {
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct Clickable;

pub fn mouse_hover(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_clickables: Query<With<Clickable>>,
    input_mouse: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    mut ev_clicked: EventWriter<Clicked>,
    mut prev_clicked: ResMut<PreviousClicked>,
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
            ev_clicked.send(Clicked {
                entity,
                prev_entity: prev_clicked.entity,
            });

            prev_clicked.entity = Some(entity);
            println!("Clicking entity {:?}", entity);
        }
        // true
        false
    });
}

pub fn hover_animation(
    mut q_clickables: Query<&mut Transform, With<Clickable>>,
    mut ev_clicked: EventReader<Clicked>,
) {
    for clicked in ev_clicked.read() {
        let Ok(mut transform) = q_clickables.get_mut(clicked.entity) else {
            return;
        };

        transform.scale = Vec3::splat(1.2);

        if let Some(prev_entity) = clicked.prev_entity {
            let Ok(mut transform) = q_clickables.get_mut(prev_entity) else {
                return;
            };

            transform.scale = Vec3::splat(1.0);
        }
    }
}
