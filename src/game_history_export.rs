use crate::game_history_core::{GameHistory, GameOutcome, GameRound};
use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

impl GameHistory {
    #[must_use]
    #[allow(clippy::format_push_string)]
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

    /// Export game history to JSON format.
    ///
    /// # Errors
    ///
    /// Returns an error indicating that JSON export is not implemented.
    pub fn export_to_json(&self) -> Result<String, String> {
        Err("JSON export not implemented - serde_json dependency not available".to_string())
    }

    #[must_use]
    #[allow(clippy::format_push_string)]
    pub fn export_summary_to_text(&self) -> String {
        let mut summary = String::new();
        summary.push_str("BLACKJACK GAME SUMMARY\n");
        summary.push_str("======================\n\n");
        summary.push_str(&format!(
            "Session Start: {}\n",
            self.session_start.format("%Y-%m-%d %H:%M:%S")
        ));
        summary.push_str(&format!("Total Games: {}\n", self.total_games_played));
        summary.push_str(&format!("Wins: {}\n", self.total_wins));
        summary.push_str(&format!("Losses: {}\n", self.total_losses));
        summary.push_str(&format!("Ties: {}\n", self.total_ties));
        summary.push_str(&format!("Win Rate: {:.1}%\n", self.get_win_rate()));
        summary.push_str(&format!("Net Profit: {}\n", self.get_net_profit()));
        summary.push_str(&format!("Biggest Win: {}\n", self.biggest_win));
        summary.push_str(&format!("Biggest Loss: {}\n", self.biggest_loss));
        summary.push('\n');

        if !self.rounds.is_empty() {
            summary.push_str("RECENT GAMES:\n");
            summary.push_str("-------------\n");
            let recent_count = 10.min(self.rounds.len());
            let start_index = self.rounds.len().saturating_sub(recent_count);

            for (i, round) in self.rounds[start_index..].iter().enumerate() {
                let game_number = start_index + i + 1;
                summary.push_str(&format!(
                    "Game #{}: {} - Bet: {} - {}\n",
                    game_number,
                    round.outcome,
                    round.bet_amount,
                    round.timestamp.format("%H:%M:%S")
                ));
            }
        }

        summary

    }

    /// Load game history from CSV file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn load_from_csv(file_path: &str) -> Result<Self, String> {
        if !Path::new(file_path).exists() {
            return Ok(GameHistory::new());
        }

        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read CSV file: {e}"))?;

        let mut history = GameHistory::new();
        let lines: Vec<&str> = content.lines().collect();

        // Skip header line
        for line in lines.iter().skip(1) {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 13 {
                continue; // Skip malformed lines
            }

            // Parse the fields
            let round_number = parts[0].parse::<u32>().unwrap_or(0);
            let timestamp_str = parts[1].trim();
            let bet_amount = parts[2].parse::<i32>().unwrap_or(0);
            let player_cards_str = parts[3].trim_matches('"');
            let dealer_cards_str = parts[4].trim_matches('"');
            let player_total = parts[5].parse::<i32>().unwrap_or(0);
            let dealer_total = parts[6].parse::<i32>().unwrap_or(0);
            let outcome_str = parts[7].trim();
            let money_change = parts[8].parse::<i32>().unwrap_or(0);
            let money_after = parts[9].parse::<i32>().unwrap_or(0);
            let was_double_down = parts[10].trim().eq_ignore_ascii_case("true");
            let player_busted = parts[11].trim().eq_ignore_ascii_case("true");
            let dealer_busted = parts[12].trim().eq_ignore_ascii_case("true");

            // Parse timestamp
            let timestamp = DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S").map_or_else(|_| Local::now(), |dt| dt.with_timezone(&Local));

            // Parse outcome
            let outcome = match outcome_str {
                "Player Win" => GameOutcome::PlayerWin,
                "Dealer Win" => GameOutcome::DealerWin,
                "Tie" => GameOutcome::Tie,
                "Player Bust" => GameOutcome::PlayerBust,
                "Dealer Bust" => GameOutcome::DealerBust,
                _ => {
                    eprintln!("Warning: Unknown outcome '{outcome_str}', defaulting to Dealer Win");
                    GameOutcome::DealerWin
                }
            };

            // Parse cards
            let player_cards: Vec<String> = if player_cards_str.is_empty() {
                Vec::new()
            } else {
                player_cards_str.split(" | ").map(std::string::ToString::to_string).collect()
            };

            let dealer_cards: Vec<String> = if dealer_cards_str.is_empty() {
                Vec::new()
            } else {
                dealer_cards_str.split(" | ").map(std::string::ToString::to_string).collect()
            };

            let round = GameRound {
                round_number,
                timestamp,
                bet_amount,
                player_cards,
                dealer_cards,
                player_total,
                dealer_total,
                outcome,
                money_change,
                money_after,
                was_double_down,
                player_busted,
                dealer_busted,
            };

            history.add_round(round);
        }

        Ok(history)
    }
}
