use bevy::{
    prelude::*,
    utils::{synccell::SyncCell, HashMap},
};
use rand::{rngs::ThreadRng, Rng};
use std::fs;

use crate::emoji_ui;

#[derive(Event)]
pub struct PlaySound;

#[derive(Event)]
pub struct GenerateRandomNumber;

#[derive(Default)]
pub struct EmojiData {
    pub audio_handle: Handle<AudioSource>,
    pub vector_handle: Handle<bevy_vello::VelloVector>,
}

#[derive(Resource, Default)]
pub struct EmojiMap {
    pub names: Vec<String>,
    pub data: Vec<EmojiData>,
}

#[derive(Component)]
pub struct EmojiAudio;

pub fn load_emoji_data(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut emoji_map: ResMut<EmojiMap>,
) {
    let audio_folder = "emoji-sound-ogg/";
    let vector_folder = "emoji/";

    let Ok(audio_entries) = fs::read_dir("./assets/".to_string() + audio_folder) else {
        return;
    };

    let Ok(mut vector_entires) = fs::read_dir("./assets/".to_string() + vector_folder) else {
        return;
    };

    for audio_entry in audio_entries {
        let audio_entry: fs::DirEntry = audio_entry.unwrap();
        let vector_entry: fs::DirEntry = vector_entires.next().unwrap().unwrap();

        println!("{:#?}", audio_entry.file_name());
        println!("{:#?}", vector_entry.file_name());

        let audio_path = audio_folder.to_owned() + audio_entry.file_name().to_str().unwrap();
        let vector_path = vector_folder.to_owned() + vector_entry.file_name().to_str().unwrap();

        let audio_handle: Handle<AudioSource> = asset_server.load(audio_path);
        let vector_handle: Handle<bevy_vello::VelloVector> = asset_server.load(vector_path);

        let name: String = audio_entry.file_name().into_string().unwrap();

        emoji_map
            .names
            .push(name.split('.').next().unwrap().to_owned());

        emoji_map.data.push(EmojiData {
            audio_handle,
            vector_handle,
        })
    }

    for _ in 0..4 {
        commands
            .spawn(AudioBundle { ..default() })
            .insert(EmojiAudio);
    }
}

#[derive(Resource, Default)]
pub struct RandomNumber {
    pub numbers: [usize; 4],
}

pub fn generate_random_num(
    mut random_number: ResMut<RandomNumber>,
    mut ev_generate_random_number: EventReader<GenerateRandomNumber>,
) {
    let mut rng = rand::thread_rng();
    for _ in ev_generate_random_number.read() {
        for i in 0..4 {
            let mut random_value: usize;
            // Make sure no same random value!
            loop {
                random_value = rng.gen_range(0..25);

                if emoji_ui::array_contain_number(&random_number.numbers, random_value) == false {
                    break;
                }
            }
            println!("Random value: {}", random_value);
            random_number.numbers[i] = random_value;
        }
    }
}

pub fn play_audio(
    mut commands: Commands,
    mut ev_play_sound: EventReader<PlaySound>,
    emoji_entity: Query<Entity, With<EmojiAudio>>,
    random_num: Res<RandomNumber>,
    emoji_map: Res<EmojiMap>,
) {
    for _ in ev_play_sound.read() {
        let mut index: usize = 0;
        for entity in emoji_entity.iter() {
            commands.entity(entity).despawn();

            commands.spawn((
                AudioBundle {
                    source: emoji_map.data[random_num.numbers[index]]
                        .audio_handle
                        .clone(),
                    ..default()
                },
                EmojiAudio,
            ));
            index += 1;
        }
    }
}
