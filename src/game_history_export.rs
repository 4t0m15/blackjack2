use crate::game_history_core::GameHistory;

impl GameHistory {
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

    pub fn export_to_json(&self) -> Result<String, String> {
        Err("JSON export not implemented - serde_json dependency not available".to_string())
    }

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
        summary.push_str("\n");

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
}
