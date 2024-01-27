use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;

mod board;
mod emoji;
mod game;
mod mouse;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_vello::VelloPlugin)
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(emoji::EmojiMap::default())
        .add_systems(Startup, (setup, board::setup, emoji::load_emoji_data))
        .add_systems(
            Update,
            (
                board::setup_animation_update,
                (
                    mouse::mouse_hover,
                    mouse::hover_animation,
                    mouse::clear_hover,
                )
                    .chain(),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
