use crate::game_history::GameHistory;
use crate::text_handler;
use crate::formatting::BoxFormatter;
use std::io::{self, Write};

pub struct HistoryMenu<'a> {
    history: &'a GameHistory,
}

impl<'a> HistoryMenu<'a> {
    #[must_use]
    pub fn new(history: &'a GameHistory) -> Self {
        HistoryMenu { history }
    }

    pub fn show_menu(&self) {
        loop {
            Self::print_history_menu();
            let Ok(input) = text_handler::read_menu_input() else {
                break;
            };

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

    fn print_history_menu() {
        let mut formatter = BoxFormatter::new(40, "GAME RECORDS");
        formatter.add_line("(s) Show summary statistics");
        formatter.add_line("(r) Show recent games");
        formatter.add_line("(d) Show detailed game");
        formatter.add_line("(e) Export to CSV");
        formatter.add_line("(b) Back to main menu");
        
        println!("\n{}", formatter.build());
        print!("Choose an option: ");
        io::stdout().flush().ok();
    }

    fn show_history_summary(&self) {
        if self.history.rounds.is_empty() {
            println!("No game history available. Play some games first!");
        } else {
            self.history.display_summary();
        }
        Self::wait_for_enter();
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
        Self::wait_for_enter();
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
        Self::wait_for_enter();
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
                format!("{filename}.csv")
            };

            match std::fs::write(&full_filename, csv_content) {
                Ok(()) => {
                    println!("✓ History exported successfully to: {full_filename}");
                    println!("  Total games exported: {}", self.history.rounds.len());
                }
                Err(e) => {
                    println!("✗ Failed to export history: {e}");
                }
            }
        }
        Self::wait_for_enter();
    }

    fn wait_for_enter() {
        println!("\nPress Enter to continue...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
    }
}

pub fn show_history_menu(history: &GameHistory) {
    let menu = HistoryMenu::new(history);
    menu.show_menu();
}
