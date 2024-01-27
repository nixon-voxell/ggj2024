use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

mod board;
mod emoji;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_vello::VelloPlugin)
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .insert_resource(emoji::EmojiMap::default())
        .add_systems(Startup, (setup, board::setup, emoji::load_emoji_data))
        .add_systems(Update, board::setup_animation_update)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
