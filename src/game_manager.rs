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
    pub fn new() -> Self {
        GameManager {
            game_state: None,
            history: GameHistory::new(),
        }
    }

    pub fn run(&mut self) {
        let show_splash = text_handler::ShowSplash;
        show_splash.doit();

        loop {
            text_handler::print_menu();
            let input = text_handler::read_menu_input();
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
    }

    fn start_new_game(&mut self) {
        if self.game_state.is_none() {
            self.game_state = Some(GameState::new());
        }

        if let Some(ref mut state) = self.game_state {
            state.history = self.history.clone();
            start_blackjack_with_state(state);
            self.history = state.history.clone();
        }
    }

    pub fn get_history(&self) -> &GameHistory {
        &self.history
    }

    pub fn get_win_rate(&self) -> f64 {
        self.history.get_win_rate()
    }

    pub fn get_total_games(&self) -> u32 {
        self.history.total_games_played
    }

    pub fn get_net_profit(&self) -> i32 {
        self.history.get_net_profit()
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}
