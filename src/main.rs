use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;
use motiongfx_typst::TypstCompilerPlugin;
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
        .add_plugins((
            MotionGfx,
            MotionGfxBevy,
            MotionGfxVello,
            TypstCompilerPlugin::new(Vec::new()),
        ))
        .add_plugins(bevy_vello::VelloPlugin)
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
        .add_systems(PreStartup, emoji::load_emoji_data)
        .add_systems(Startup, setup)
        .add_systems(Startup, menu_ui::menu_button)
        .add_systems(Startup, (emoji_ui::setup, emoji_ui::setup_menu))
        .add_systems(Startup, store_four_random_values)
        .add_systems(Startup, emoji::load_emoji_data)
        .add_systems(
            Update,
            (
                // board::setup_animation_update,
                setup_animation_update,
                menu_ui::start_button_evt,
                mouse::mouse_hover,
                mouse::hover_animation,
                game::game_manager,
                store_four_random_values,
                print_random_numbers,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct SetupTimeline;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_animation_update(
    mut q_timelines: Query<&mut Timeline, With<SetupTimeline>>,
    q_sequences: Query<&Sequence>,
    time: Res<Time>,
) {
    for mut timeline in q_timelines.iter_mut() {
        let Ok(sequence) = q_sequences.get(timeline.sequence_id().unwrap()) else {
            continue;
        };

        // stops updating when timeline reaches the end
        if (timeline.time_scale > 0.0 && timeline.target_time >= sequence.duration())
            || (timeline.time_scale < 0.0 && timeline.target_time <= 0.0)
        {
            continue;
        }

        timeline.target_time += timeline.time_scale * time.delta_seconds();
    }
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
