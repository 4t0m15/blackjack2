pub mod art_handler;
pub mod card_handler;
pub mod deck_manager;
pub mod enemy_ai_handler;
pub mod game_data;
pub mod game_history;
pub mod game_history_core;
pub mod game_history_display;
pub mod game_history_export;
pub mod game_loop;
pub mod game_manager;
pub mod game_state;
pub mod history_menu;
pub mod main_menu;
pub mod menu_handling;
pub mod player_handler;
pub mod save_system;
pub mod text_handler;
pub mod win_checker;

use game_manager::GameManager;

fn main() {
    let mut game_manager = GameManager::new();
    game_manager.run();
}