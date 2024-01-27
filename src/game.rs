use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

use crate::emoji_ui;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Start,
    InGame,
    End,
}

#[derive(Resource, Default)]
pub struct GameStateRes {
    pub curr_state: GameState,
    pub target_state: GameState,
}

pub fn game_manager(
    mut game_state: ResMut<GameStateRes>,
    mut q_emoji_ui_setup: Query<&mut Timeline, With<emoji_ui::TileSetupTimeline>>,
) {
    // Game state already achieved
    if game_state.curr_state == game_state.target_state {
        return;
    }

    let Ok(mut emoji_ui_setup) = q_emoji_ui_setup.get_single_mut() else {
        return;
    };

    match game_state.target_state {
        GameState::Start => {}
        GameState::InGame => emoji_ui_setup.time_scale = 1.0,
        GameState::End => {}
    }

    // Update curr state to target state
    game_state.curr_state = game_state.target_state;
}
