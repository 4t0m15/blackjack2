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

    pub fn get_win_rate(&self) -> f64 {
        if self.total_games_played == 0 {
            0.0
        } else {
            (self.total_wins as f64 / self.total_games_played as f64) * 100.0
        }
    }

    pub fn get_net_profit(&self) -> i32 {
        self.total_money_won - self.total_money_lost
    }

    pub fn display_summary(&self) {
        println!("\n╔══════════════════════════════════════╗");
        println!("║           GAME HISTORY SUMMARY       ║");
        println!("╠══════════════════════════════════════╣");
        println!(
            "║ Session Start: {}     ║",
            self.session_start.format("%H:%M:%S")
        );
        println!("║ Total Games: {:24} ║", self.total_games_played);
        println!("║ Wins: {:31} ║", self.total_wins);
        println!("║ Losses: {:29} ║", self.total_losses);
        println!("║ Ties: {:31} ║", self.total_ties);
        println!("║ Win Rate: {:25.1}% ║", self.get_win_rate());
        println!("║                                      ║");
        println!("║ Money Won: {:26} ║", self.total_money_won);
        println!("║ Money Lost: {:25} ║", self.total_money_lost);
        println!("║ Net Profit: {:25} ║", self.get_net_profit());
        println!("║ Biggest Win: {:24} ║", self.biggest_win);
        println!("║ Biggest Loss: {:23} ║", self.biggest_loss);
        println!("╚══════════════════════════════════════╝\n");
    }

    pub fn display_recent_games(&self, count: usize) {
        let recent_count = count.min(self.rounds.len());
        if recent_count == 0 {
            println!("No games played yet in this session.\n");
            return;
        }

        println!("\n╔════════════════════════════════════════════════════════════════╗");
        println!(
            "║                    RECENT GAMES (Last {})                      ║",
            recent_count
        );
        println!("╠════════════════════════════════════════════════════════════════╣");

        let start_index = self.rounds.len().saturating_sub(recent_count);
        for (i, round) in self.rounds[start_index..].iter().enumerate() {
            let display_number = start_index + i + 1;
            println!(
                "║ Game #{:<3} │ {:12} │ Bet: {:3} │ {:19} ║",
                display_number,
                round.outcome,
                round.bet_amount,
                round.timestamp.format("%H:%M:%S")
            );

            println!(
                "║         │ Player: {:25} ({:2}) ║",
                format_cards_short(&round.player_cards),
                round.player_total
            );

            println!(
                "║         │ Dealer: {:25} ({:2}) ║",
                format_cards_short(&round.dealer_cards),
                round.dealer_total
            );

            let money_display = if round.money_change >= 0 {
                format!("+{}", round.money_change)
            } else {
                format!("{}", round.money_change)
            };

            println!(
                "║         │ Money: {:8} │ Total: {:14} ║",
                money_display, round.money_after
            );

            if i < recent_count - 1 {
                println!("║         ├─────────────────────────────────────────────────── ║");
            }
        }

        println!("╚════════════════════════════════════════════════════════════════╝\n");
    }

    pub fn display_detailed_game(&self, round_number: usize) {
        if round_number == 0 || round_number > self.rounds.len() {
            println!(
                "Invalid game number. Please choose a number between 1 and {}.",
                self.rounds.len()
            );
            return;
        }

        let round = &self.rounds[round_number - 1];

        println!("\n╔══════════════════════════════════════════════════════════════════╗");
        println!(
            "║                       GAME #{:<3} DETAILS                        ║",
            round_number
        );
        println!("╠══════════════════════════════════════════════════════════════════╣");
        println!(
            "║ Time: {}                                         ║",
            round.timestamp.format("%Y-%m-%d %H:%M:%S")
        );
        println!("║ Bet Amount: {:52} ║", round.bet_amount);
        if round.was_double_down {
            println!("║ Double Down: Yes                                                 ║");
        }
        println!("║                                                                  ║");
        println!(
            "║ Player Cards: {:50} ║",
            format_cards_long(&round.player_cards)
        );
        println!("║ Player Total: {:50} ║", round.player_total);
        if round.player_busted {
            println!("║ Player Status: BUSTED                                           ║");
        }
        println!("║                                                                  ║");
        println!(
            "║ Dealer Cards: {:50} ║",
            format_cards_long(&round.dealer_cards)
        );
        println!("║ Dealer Total: {:50} ║", round.dealer_total);
        if round.dealer_busted {
            println!("║ Dealer Status: BUSTED                                           ║");
        }
        println!("║                                                                  ║");
        println!("║ Outcome: {:59} ║", round.outcome);

        let money_display = if round.money_change >= 0 {
            format!("+{}", round.money_change)
        } else {
            format!("{}", round.money_change)
        };

        println!("║ Money Change: {:54} ║", money_display);
        println!("║ Money After: {:55} ║", round.money_after);
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
    }

    pub fn export_to_csv(&self) -> String {
        let mut csv = String::new();
        csv.push_str("Round,Timestamp,Bet,PlayerCards,DealerCards,PlayerTotal,DealerTotal,Outcome,MoneyChange,MoneyAfter,DoubleDown,PlayerBust,DealerBust\n");

        for (i, round) in self.rounds.iter().enumerate() {
            csv.push_str(&format!(
                "{},{},{},\"{}\",\"{}\",{},{},{},{},{},{},{},{}\n",
                i + 1,
                round.timestamp.format("%Y-%m-%d %H:%M:%S"),
                round.bet_amount,
                round.player_cards.join(" | "),
                round.dealer_cards.join(" | "),
                round.player_total,
                round.dealer_total,
                round.outcome,
                round.money_change,
                round.money_after,
                round.was_double_down,
                round.player_busted,
                round.dealer_busted
            ));
        }

        csv
    }
}

impl Default for GameHistory {
    fn default() -> Self {
        Self::new()
    }
}

fn format_cards_short(cards: &[String]) -> String {
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

fn format_cards_long(cards: &[String]) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_history() {
        let history = GameHistory::new();
        assert_eq!(history.total_games_played, 0);
        assert_eq!(history.total_wins, 0);
        assert_eq!(history.get_win_rate(), 0.0);
    }

    #[test]
    fn test_add_round() {
        let mut history = GameHistory::new();
        let round = GameRound {
            round_number: 1,
            timestamp: Local::now(),
            bet_amount: 5,
            player_cards: vec!["K Hearts".to_string(), "A Spades".to_string()],
            dealer_cards: vec!["10 Clubs".to_string(), "8 Diamonds".to_string()],
            player_total: 21,
            dealer_total: 18,
            outcome: GameOutcome::PlayerWin,
            money_change: 10,
            money_after: 20,
            was_double_down: false,
            player_busted: false,
            dealer_busted: false,
        };

        history.add_round(round);
        assert_eq!(history.total_games_played, 1);
        assert_eq!(history.total_wins, 1);
        assert_eq!(history.get_win_rate(), 100.0);
    }

    #[test]
    fn test_format_cards_short() {
        let cards = vec!["A Hearts".to_string(), "K Spades".to_string()];
        let formatted = format_cards_short(&cards);
        assert_eq!(formatted, "AH KS");
    }
}
