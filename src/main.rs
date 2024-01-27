use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;
// use motiongfx_typst::{TypstCompiler, TypstCompilerPlugin};
// use motiongfx_vello::{bevy_vello_renderer::vello::peniko, svg};
use bevy::prelude::ResMut;
// use bevy_prng::ChaCha8Rng;
// use bevy_rand::prelude::*;
// use rand_core::RngCore;
// mod board;
use rand::{rngs::ThreadRng, Rng};

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
        // .add_plugins(RapierDebugRenderPlugin::default())
        // Resources
        .insert_resource(mouse::PreviousClicked::default())
        .insert_resource(emoji::EmojiMap::default())
        .insert_resource(game::GameStateRes::default())
        // .insert_resource(game::GameStateRes {
        //     curr_state: game::GameState::Start,
        //     target_state: game::GameState::InGame,
        // })
        .add_event::<mouse::Clicked>()
        // .add_systems(Startup, (setup, board::setup))
        // Systems
        .add_systems(Startup, setup)
        .add_systems(Startup, menu_ui::menu_button)
        .add_systems(
            Startup,
            (
                emoji_ui::setup,
                emoji_ui::setup_menu,
                store_four_random_values,
            ),
        )
        .add_systems(Startup, emoji::load_emoji_data)
        .add_systems(
            Update,
            (
                // board::setup_animation_update,
                emoji_ui::setup_animation_update,
                mouse::mouse_hover,
                mouse::hover_animation,
                game::game_manager,
                store_four_random_values,
                print_random_numbers,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

// #[derive(Resource)]
// struct RandomNumber(u32);

// fn store_random_value(mut commands: Commands) {
//     let mut rng = rand::thread_rng();
//     let random_value = rng.gen_range(0..25);

//     let random_number = RandomNumber(random_value);

//     println!("Integer: {}", random_value);

//     commands.insert_resource(random_number);
// }

// fn generate_four_random_numbers(mut commands: Commands, random_numbers: ResMut<RandomNumber>) {
//     for _ in 0..4 {
//         let random_number = random_numbers.0;

//         println!("Generated Random Number: {}", random_number);
//     }
// }

#[derive(Component)]
struct RandomNumber(u32);

fn store_four_random_values(mut commands: Commands) {
    for _ in 0..4 {
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0..25);

        let random_number = RandomNumber(random_value);

        // println!("Generated Random Number: {}", random_value);

        commands.spawn_empty().insert(random_number);
    }
}

fn print_random_numbers(query: Query<&RandomNumber>) {
    for random_number in query.iter() {
        println!("Stored Random Number: {}", random_number.0);
    }
}
