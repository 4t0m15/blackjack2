use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone)]
pub struct GameRound {
    pub round_number: u32,
    pub timestamp: DateTime<Local>,
    pub bet_amount: i32,
    pub player_cards: Vec<String>,
    pub dealer_cards: Vec<String>,
    pub player_total: i32,
    pub dealer_total: i32,
    pub outcome: GameOutcome,
    pub money_change: i32,
    pub money_after: i32,
    pub was_double_down: bool,
    pub player_busted: bool,
    pub dealer_busted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameOutcome {
    PlayerWin,
    DealerWin,
    Tie,
    PlayerBust,
    DealerBust,
}

impl fmt::Display for GameOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameOutcome::PlayerWin => write!(f, "Player Win"),
            GameOutcome::DealerWin => write!(f, "Dealer Win"),
            GameOutcome::Tie => write!(f, "Tie"),
            GameOutcome::PlayerBust => write!(f, "Player Bust"),
            GameOutcome::DealerBust => write!(f, "Dealer Bust"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameHistory {
    pub rounds: Vec<GameRound>,
    pub session_start: DateTime<Local>,
    pub total_games_played: u32,
    pub total_wins: u32,
    pub total_losses: u32,
    pub total_ties: u32,
    pub total_money_won: i32,
    pub total_money_lost: i32,
    pub biggest_win: i32,
    pub biggest_loss: i32,
}

impl GameHistory {
    #[must_use]
    pub fn new() -> Self {
        GameHistory {
            rounds: Vec::new(),
            session_start: Local::now(),
            total_games_played: 0,
            total_wins: 0,
            total_losses: 0,
            total_ties: 0,
            total_money_won: 0,
            total_money_lost: 0,
            biggest_win: 0,
            biggest_loss: 0,
        }
    }

    pub fn add_round(&mut self, round: GameRound) {
        self.total_games_played += 1;

        match round.outcome {
            GameOutcome::PlayerWin | GameOutcome::DealerBust => {
                self.total_wins += 1;
                self.total_money_won += round.money_change;
                if round.money_change > self.biggest_win {
                    self.biggest_win = round.money_change;
                }
            }
            GameOutcome::DealerWin | GameOutcome::PlayerBust => {
                self.total_losses += 1;
                self.total_money_lost += round.money_change.abs();
                if round.money_change.abs() > self.biggest_loss {
                    self.biggest_loss = round.money_change.abs();
                }
            }
            GameOutcome::Tie => {
                self.total_ties += 1;
            }
        }

        self.rounds.push(round);
    }

    #[must_use]
    pub fn get_win_rate(&self) -> f64 {
        if self.total_games_played == 0 {
            0.0
        } else {
            (f64::from(self.total_wins) / f64::from(self.total_games_played)) * 100.0
        }
    }

    #[must_use]
    pub fn get_net_profit(&self) -> i32 {
        self.total_money_won - self.total_money_lost
    }
}

impl Default for GameHistory {
    fn default() -> Self {
        Self::new()
    }
}
