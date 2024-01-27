use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;

mod board;
mod emoji;
mod emoji_ui;
mod game;
mod menu_ui;
mod mouse;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_vello::VelloPlugin)
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // Resources
        .insert_resource(mouse::PreviousClicked::default())
        .insert_resource(emoji::EmojiMap::default())
        .insert_resource(game::GameStateRes::default())
        .add_event::<mouse::Clicked>()
        // .add_systems(Startup, (setup, board::setup))
        // Systems
        .add_systems(Startup, (setup, emoji_ui::setup, menu_ui::menu_button))
        .add_systems(Startup, emoji::load_emoji_data)
        .add_systems(
            Update,
            (
                // board::setup_animation_update,
                emoji_ui::setup_animation_update,
                mouse::mouse_hover,
                mouse::hover_animation,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
