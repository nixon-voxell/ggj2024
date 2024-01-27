use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;

mod board;
mod emoji_ui;
mod game;
mod mouse;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // .add_systems(Startup, (setup, board::setup))
        .add_systems(Startup, (setup, emoji_ui::setup))
        .add_systems(
            Update,
            (
                // board::setup_animation_update,
                emoji_ui::setup_animation_update,
                (mouse::clear_hover, mouse::mouse_hover).chain(),
                mouse::hover_animation,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
