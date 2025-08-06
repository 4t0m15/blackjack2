use crate::game_history::GameHistory;

pub struct GameState {
    pub card_deck: Vec<String>,
    pub player_cards: Vec<String>,
    pub dealer_cards: Vec<String>,
    pub money: i32,
    pub bet: i32,
    pub games_won: i32,
    pub games_lost: i32,
    pub player_card_count: i32,
    pub dealer_card_count: i32,
    pub deck_index: i32,
    pub history: GameHistory,
    pub current_round_start_money: i32,
    pub was_double_down: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    #[must_use]
    pub fn new() -> Self {
        let save_data = crate::save_system::load_save_data();
        
        GameState {
            card_deck: Vec::new(),
            player_cards: Vec::new(),
            dealer_cards: Vec::new(),
            money: save_data.money,
            bet: 0,
            games_won: save_data.games_won,
            games_lost: save_data.games_lost,
            player_card_count: 0,
            dealer_card_count: 0,
            deck_index: 0,
            history: GameHistory::new(),
            current_round_start_money: save_data.money,
            was_double_down: false,
        }
    }
    
    /// Save the current game state to disk
    pub fn save_to_disk(&self) {
        let save_data = crate::save_system::create_save_data(
            self.money,
            self.games_won,
            self.games_lost,
        );
        crate::save_system::auto_save(&save_data);
    }
}
