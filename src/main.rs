use bevy::prelude::*;
use bevy_motiongfx::prelude::*;
use bevy_rapier2d::prelude::*;
use motiongfx_typst::TypstCompilerPlugin;

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
        .insert_resource(emoji_ui::PlacementIndex(0))
        .insert_resource(emoji_ui::EmojiGuesses::default())
        .add_event::<mouse::Clicked>()
        .add_event::<emoji::PlaySound>()
        // .add_systems(Startup, (setup, board::setup))
        // Systems
        .add_systems(PreStartup, emoji::load_emoji_data)
        .add_systems(Startup, setup)
        .add_systems(Startup, menu_ui::menu_button)
        .add_systems(
            Startup,
            (
                emoji_ui::setup,
                emoji_ui::setup_menu,
                emoji_ui::setup_play_sound_btn,
                emoji::generate_random_num,
            ),
        )
        .add_systems(
            Update,
            (
                // board::setup_animation_update,
                setup_animation_update,
                menu_ui::start_button_evt,
                mouse::mouse_hover,
                mouse::hover_animation,
                game::game_manager,
                // store_four_random_values,
                // print_random_numbers,
            ),
        )
        .add_systems(
            Update,
            (
                emoji_ui::play_sound_button_evt,
                emoji_ui::placement_tiles_evt,
                emoji_ui::emoji_tiles_evt,
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
