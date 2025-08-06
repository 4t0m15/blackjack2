//! A console-based Blackjack game written in Rust
//!
//! This crate provides a complete blackjack implementation with game history tracking,
//! ASCII art cards, and a menu-driven interface.

pub mod art_handler;
pub mod card_handler;
pub mod enemy_ai_handler;
pub mod game_history;
pub mod game_manager;
pub mod main_menu;
pub mod menu_handling;
pub mod player_handler;
pub mod text_handler;

// Re-export commonly used types for convenience
pub use game_history::{GameHistory, GameOutcome, GameRound};
pub use game_manager::GameManager;
pub use card_handler::GameState;
