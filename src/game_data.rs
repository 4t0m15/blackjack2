use chrono::{DateTime, Local};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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

impl GameRound {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        round_number: u32,
        bet_amount: i32,
        player_cards: Vec<String>,
        dealer_cards: Vec<String>,
        player_total: i32,
        dealer_total: i32,
        outcome: GameOutcome,
        money_change: i32,
        money_after: i32,
        was_double_down: bool,
    ) -> Self {
        GameRound {
            round_number,
            timestamp: Local::now(),
            bet_amount,
            player_cards,
            dealer_cards,
            player_total,
            dealer_total,
            outcome,
            money_change,
            money_after,
            was_double_down,
            player_busted: player_total > 21,
            dealer_busted: dealer_total > 21,
        }
    }

    #[must_use]
    pub fn is_win(&self) -> bool {
        matches!(
            self.outcome,
            GameOutcome::PlayerWin | GameOutcome::DealerBust
        )
    }

    #[must_use]
    pub fn is_loss(&self) -> bool {
        matches!(
            self.outcome,
            GameOutcome::DealerWin | GameOutcome::PlayerBust
        )
    }

    #[must_use]
    pub fn is_tie(&self) -> bool {
        matches!(self.outcome, GameOutcome::Tie)
    }

    #[must_use]
    pub fn get_display_outcome(&self) -> String {
        match self.outcome {
            GameOutcome::PlayerWin => "🎉 WIN".to_string(),
            GameOutcome::DealerWin => "❌ LOSS".to_string(),
            GameOutcome::Tie => "🤝 TIE".to_string(),
            GameOutcome::PlayerBust => "💥 BUST".to_string(),
            GameOutcome::DealerBust => "🎯 DEALER BUST".to_string(),
        }
    }

    #[must_use]
    pub fn format_cards_short(&self, cards: &[String]) -> String {
        if cards.len() <= 3 {
            cards
                .iter()
                .map(|card| {
                    let parts: Vec<&str> = card.split_whitespace().collect();
                    if parts.len() >= 2 {
                        format!("{}{}", parts[0], parts[1].chars().next().unwrap_or('?'))
                    } else {
                        "??".to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            let visible: Vec<String> = cards
                .iter()
                .take(2)
                .map(|card| {
                    let parts: Vec<&str> = card.split_whitespace().collect();
                    if parts.len() >= 2 {
                        format!("{}{}", parts[0], parts[1].chars().next().unwrap_or('?'))
                    } else {
                        "??".to_string()
                    }
                })
                .collect();
            format!("{} +{}", visible.join(" "), cards.len() - 2)
        }
    }

    #[must_use]
    pub fn format_cards_long(&self, cards: &[String]) -> String {
        cards
            .iter()
            .map(|card| {
                let parts: Vec<&str> = card.split_whitespace().collect();
                if parts.len() >= 2 {
                    format!("{} of {}", parts[0], parts[1])
                } else {
                    card.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
