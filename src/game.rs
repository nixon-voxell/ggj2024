use bevy::prelude::*;
use bevy_motiongfx::prelude::*;

use crate::{emoji_ui, menu_ui};

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
    mut q_emoji_ui_setup: Query<
        &mut Timeline,
        (
            With<emoji_ui::TileSetupTimeline>,
            Without<menu_ui::MenuSetupTimeline>,
        ),
    >,
    mut q_menu_ui_setup: Query<
        &mut Timeline,
        (
            With<menu_ui::MenuSetupTimeline>,
            Without<emoji_ui::TileSetupTimeline>,
        ),
    >,
) {
    // Game state already achieved
    if game_state.curr_state == game_state.target_state {
        return;
    }

    match game_state.target_state {
        GameState::Start => {
            for mut emoji_ui_setup in q_emoji_ui_setup.iter_mut() {
                emoji_ui_setup.time_scale = -1.0;
            }

            for mut menu_ui_setup in q_menu_ui_setup.iter_mut() {
                menu_ui_setup.time_scale = 1.0;
            }
        }
        GameState::InGame => {
            for mut emoji_ui_setup in q_emoji_ui_setup.iter_mut() {
                emoji_ui_setup.time_scale = 1.0;
            }

            for mut menu_ui_setup in q_menu_ui_setup.iter_mut() {
                menu_ui_setup.time_scale = -1.0;
            }
        }
        GameState::End => {}
    }

    // Update curr state to target state
    game_state.curr_state = game_state.target_state;
}
