use bevy::{prelude::*, utils::HashMap};
use rand::{rngs::ThreadRng, Rng};
use std::fs;

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
    pub map: HashMap<String, EmojiData>,
}

pub fn load_emoji_data(asset_server: Res<AssetServer>, mut emoji_map: ResMut<EmojiMap>) {
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

        // println!("{}", name);
        emoji_map.map.insert(
            name.split('.').next().unwrap().to_owned(),
            EmojiData {
                audio_handle,
                vector_handle,
            },
        );
    }
}

#[derive(Resource)]
pub struct RandomNumber {
    pub numbers: [usize; 4],
}

pub fn generate_random_num(
    mut random_number: ResMut<RandomNumber>,
    mut ev_generate_random_number: EventReader<GenerateRandomNumber>,
) {
    for _ in ev_generate_random_number.read() {
        for i in 0..4 {
            let mut rng = rand::thread_rng();
            let random_value = rng.gen_range(0..25);
            random_number.numbers[i] = random_value;
        }
    }
}
