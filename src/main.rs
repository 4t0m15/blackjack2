pub mod art_handler;
pub mod card_handler;
pub mod enemy_ai_handler;
pub mod game_history;
pub mod game_manager;
pub mod main_menu;
pub mod menu_handling;
pub mod player_handler;
pub mod text_handler;

use game_manager::GameManager;

fn main() {
    let mut game_manager = GameManager::new();
    game_manager.run();
}
