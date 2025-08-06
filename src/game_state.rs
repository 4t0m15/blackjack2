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
        GameState {
            card_deck: Vec::new(),
            player_cards: Vec::new(),
            dealer_cards: Vec::new(),
            money: 10,
            bet: 0,
            games_won: 0,
            games_lost: 0,
            player_card_count: 0,
            dealer_card_count: 0,
            deck_index: 0,
            history: GameHistory::new(),
            current_round_start_money: 10,
            was_double_down: false,
        }
    }
}
