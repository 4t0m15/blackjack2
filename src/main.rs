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
use std::io::{self, Write};
use std::panic;
use std::process;

fn main() {
    setup_panic_handler();

    match setup_and_run_game() {
        Ok(_) => {}
        Err(_) => {
            process::exit(1);
        }
    }
}

fn setup_and_run_game() -> Result<(), String> {
    let mut game_manager = GameManager::new();

    game_manager.run();

    Ok(())
}

fn setup_panic_handler() {
    panic::set_hook(Box::new(|panic_info| {
        eprintln!("\n\n====== GAME CRASHED ======");
        eprintln!("We apologize for the inconvenience!");

        if let Some(location) = panic_info.location() {
            eprintln!("Location: {}:{}", location.file(), location.line());
        }

        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("Error: {}", s);
        } else {
            eprintln!("An unknown error occurred");
        }

        eprintln!("\nPlease report this issue at https://github.com/4t0m15/blackjack2");
        eprintln!("Press Enter to exit...");

        let mut buffer = String::new();
        io::stdout().flush().expect("Failed to flush stdout");
        let _ = io::stdin().read_line(&mut buffer);

        process::exit(1);
    }));
}
