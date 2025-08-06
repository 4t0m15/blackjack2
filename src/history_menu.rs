use crate::game_history::GameHistory;
use crate::text_handler;
use std::io::{self, Write};

pub struct HistoryMenu<'a> {
    history: &'a GameHistory,
}

impl<'a> HistoryMenu<'a> {
    pub fn new(history: &'a GameHistory) -> Self {
        HistoryMenu { history }
    }

    pub fn show_menu(&self) {
        loop {
            self.print_history_menu();
            let input = text_handler::read_menu_input();

            match input.trim().to_lowercase().as_str() {
                "s" => self.show_history_summary(),
                "r" => self.show_recent_games(),
                "d" => self.show_detailed_game(),
                "e" => self.export_history(),
                "b" | "back" => break,
                _ => {
                    println!("Invalid option. Please try again.");
                }
            }
        }
    }

    fn print_history_menu(&self) {
        println!("\n╔══════════════════════════════════════╗");
        println!("║            GAME RECORDS              ║");
        println!("╠══════════════════════════════════════╣");
        println!("║ (s) Show summary statistics          ║");
        println!("║ (r) Show recent games                ║");
        println!("║ (d) Show detailed game               ║");
        println!("║ (e) Export to CSV                    ║");
        println!("║ (b) Back to main menu                ║");
        println!("╚══════════════════════════════════════╝");
        print!("Choose an option: ");
        io::stdout().flush().ok();
    }

    fn show_history_summary(&self) {
        if self.history.rounds.is_empty() {
            println!("No game history available. Play some games first!");
        } else {
            self.history.display_summary();
        }
        self.wait_for_enter();
    }

    fn show_recent_games(&self) {
        if self.history.rounds.is_empty() {
            println!("No game history available. Play some games first!");
        } else {
            print!("How many recent games to show (default 5): ");
            io::stdout().flush().ok();

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();

            let count = input.trim().parse::<usize>().unwrap_or(5);
            self.history.display_recent_games(count);
        }
        self.wait_for_enter();
    }

    fn show_detailed_game(&self) {
        if self.history.rounds.is_empty() {
            println!("No games played yet!");
        } else {
            print!("Enter game number (1-{}): ", self.history.rounds.len());
            io::stdout().flush().ok();

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();

            if let Ok(game_num) = input.trim().parse::<usize>() {
                self.history.display_detailed_game(game_num);
            } else {
                println!("Invalid game number!");
            }
        }
        self.wait_for_enter();
    }

    fn export_history(&self) {
        if self.history.rounds.is_empty() {
            println!("No game history to export!");
        } else {
            let csv_content = self.history.export_to_csv();

            print!("Enter filename (without extension): ");
            io::stdout().flush().ok();

            let mut filename = String::new();
            io::stdin().read_line(&mut filename).ok();
            let filename = filename.trim();

            let full_filename = if filename.is_empty() {
                format!(
                    "blackjack_history_{}.csv",
                    chrono::Local::now().format("%Y%m%d_%H%M%S")
                )
            } else {
                format!("{}.csv", filename)
            };

            match std::fs::write(&full_filename, csv_content) {
                Ok(_) => {
                    println!("✓ History exported successfully to: {}", full_filename);
                    println!("  Total games exported: {}", self.history.rounds.len());
                }
                Err(e) => {
                    println!("✗ Failed to export history: {}", e);
                }
            }
        }
        self.wait_for_enter();
    }

    fn wait_for_enter(&self) {
        println!("\nPress Enter to continue...");
        let mut _input = String::new();
        io::stdin().read_line(&mut _input).ok();
    }
}

pub fn show_history_menu(history: &GameHistory) {
    let menu = HistoryMenu::new(history);
    menu.show_menu();
}
