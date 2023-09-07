use bevy::prelude::*;

pub const STARTING_GAME_STATE: GameState = GameState::MainMenu;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
    Paused,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_pause, start_game));
    }
}

pub fn toggle_pause(
    game_state: Res<State<GameState>>,
    keybopard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keybopard_input.just_pressed(KeyCode::Escape) {
        if game_state.get() == &GameState::Playing {
            next_state.set(GameState::Paused);
            println!("Game paused!")
        } else if game_state.get() == &GameState::Paused {
            next_state.set(GameState::Playing);
            println!("Game unpaused!")
        }
    }
}

pub fn start_game(
    game_state: Res<State<GameState>>,
    keybopard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keybopard_input.just_pressed(KeyCode::Return) && game_state.get() == &GameState::MainMenu {
        next_state.set(GameState::Playing);
        println!("Game started!")
    }
}
