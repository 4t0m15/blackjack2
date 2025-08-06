use crate::game_history::GameHistory;
use crate::game_loop::start_blackjack_with_state;
use crate::game_state::GameState;
use crate::history_menu::show_history_menu;
use crate::menu_handling::MenuAction;
use crate::text_handler;

pub struct GameManager {
    pub game_state: Option<GameState>,
    pub history: GameHistory,
}

impl GameManager {
    #[must_use]
    pub fn new() -> Self {
        // Try to load existing game history from CSV file
        let history = GameHistory::load_from_csv("stats.csv").unwrap_or_else(|e| {
            eprintln!("Warning: Could not load game history: {e}");
            GameHistory::new()
        });

        GameManager {
            game_state: None,
            history,
        }
    }

    /// Run the game manager main loop.
    ///
    /// # Errors
    ///
    /// Returns an error if there's an issue reading user input.
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let show_splash = text_handler::ShowSplash;
        show_splash.doit();

        loop {
            text_handler::print_menu();
            let input = text_handler::read_menu_input()?;
            let action = MenuAction::from_string(&input);

            match action {
                MenuAction::About => {
                    let about = crate::main_menu::PrintAbout;
                    about.doit();
                }
                MenuAction::Help => {
                    let help = crate::main_menu::PrintHelp;
                    help.show_controls();
                }
                MenuAction::Guide => {
                    let help = crate::main_menu::PrintHelp;
                    help.show_instructions();
                }
                MenuAction::NewGame => {
                    self.start_new_game();
                }
                MenuAction::History => {
                    show_history_menu(&self.history);
                }
                MenuAction::Quit => {
                    println!("Thanks for playing! Goodbye!");
                    break;
                }
                MenuAction::Invalid => {
                    text_handler::print_invalid_option();
                }
            }
        }
        Ok(())
    }

    fn start_new_game(&mut self) {
        // Create or reset the game state, but preserve history
        let mut state = if let Some(mut existing_state) = self.game_state.take() {
            // Reset game state but keep the history
            existing_state.player_cards.clear();
            existing_state.dealer_cards.clear();
            existing_state.bet = 0;
            existing_state.player_card_count = 0;
            existing_state.dealer_card_count = 0;
            existing_state.deck_index = 0;
            existing_state.was_double_down = false;
            existing_state.history = self.history.clone();
            existing_state
        } else {
            let mut new_state = GameState::new();
            new_state.history = self.history.clone();
            new_state
        };

        start_blackjack_with_state(&mut state);
        self.history = state.history.clone();
        self.game_state = Some(state);
    }

    #[must_use]
    pub fn get_history(&self) -> &GameHistory {
        &self.history
    }

    #[must_use]
    pub fn get_win_rate(&self) -> f64 {
        self.history.get_win_rate()
    }

    #[must_use]
    pub fn get_total_games(&self) -> u32 {
        self.history.total_games_played
    }

    #[must_use]
    pub fn get_net_profit(&self) -> i32 {
        self.history.get_net_profit()
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}
