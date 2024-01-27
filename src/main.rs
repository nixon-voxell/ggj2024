use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

mod board;
mod game;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .add_systems(Startup, (setup, board::setup))
        .add_systems(Update, board::setup_animation_update)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
