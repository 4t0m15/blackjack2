use crate::formatting::{format_money, format_percentage, BoxFormatter};
use crate::game_history_core::GameHistory;

impl GameHistory {
    pub fn display_summary(&self) {
        let mut formatter = BoxFormatter::new(40, "GAME HISTORY SUMMARY");

        formatter.add_field_aligned("Session Start", &self.session_start.format("%H:%M:%S"));
        formatter.add_field_aligned("Total Games", &self.total_games_played);
        formatter.add_field_aligned("Wins", &self.total_wins);
        formatter.add_field_aligned("Losses", &self.total_losses);
        formatter.add_field_aligned("Ties", &self.total_ties);
        formatter.add_field_aligned("Win Rate", &format_percentage(self.get_win_rate()));
        formatter.add_empty_line();
        formatter.add_field_aligned("Money Won", &self.total_money_won);
        formatter.add_field_aligned("Money Lost", &self.total_money_lost);
        formatter.add_field_aligned("Net Profit", &self.get_net_profit());
        formatter.add_field_aligned("Biggest Win", &self.biggest_win);
        formatter.add_field_aligned("Biggest Loss", &self.biggest_loss);

        println!("\n{}\n", formatter.build());
    }

    pub fn display_recent_games(&self, count: usize) {
        let recent_count = count.min(self.rounds.len());
        if recent_count == 0 {
            println!("No games played yet in this session.\n");
            return;
        }

        let mut formatter = BoxFormatter::new(66, &format!("RECENT GAMES (Last {recent_count})"));

        let start_index = self.rounds.len().saturating_sub(recent_count);
        for (i, round) in self.rounds[start_index..].iter().enumerate() {
            let display_number = start_index + i + 1;

            // Game header
            formatter.add_field_aligned(
                &format!("Game #{display_number}"),
                &format!(
                    "{} | Bet: {} | {}",
                    round.outcome,
                    round.bet_amount,
                    round.timestamp.format("%H:%M:%S")
                ),
            );

            // Player cards
            formatter.add_field_aligned(
                "Player",
                &format!(
                    "{} ({})",
                    format_cards_short(&round.player_cards),
                    round.player_total
                ),
            );

            // Dealer cards
            formatter.add_field_aligned(
                "Dealer",
                &format!(
                    "{} ({})",
                    format_cards_short(&round.dealer_cards),
                    round.dealer_total
                ),
            );

            // Money change
            formatter.add_field_aligned("Money Change", &format_money(round.money_change));
            formatter.add_field_aligned("Total After", &round.money_after);

            if i < recent_count - 1 {
                formatter.add_separator();
            }
        }

        println!("\n{}\n", formatter.build());
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

        let mut formatter = BoxFormatter::new(70, &format!("GAME #{round_number} DETAILS"));

        formatter.add_field_aligned("Time", &round.timestamp.format("%Y-%m-%d %H:%M:%S"));
        formatter.add_field_aligned("Bet Amount", &round.bet_amount);

        if round.was_double_down {
            formatter.add_field_aligned("Double Down", &"Yes");
        }

        formatter.add_empty_line();
        formatter.add_field_aligned("Player Cards", &format_cards_long(&round.player_cards));
        formatter.add_field_aligned("Player Total", &round.player_total);

        if round.player_busted {
            formatter.add_field_aligned("Player Status", &"BUSTED");
        }

        formatter.add_empty_line();
        formatter.add_field_aligned("Dealer Cards", &format_cards_long(&round.dealer_cards));
        formatter.add_field_aligned("Dealer Total", &round.dealer_total);

        if round.dealer_busted {
            formatter.add_field_aligned("Dealer Status", &"BUSTED");
        }

        formatter.add_empty_line();
        formatter.add_field_aligned("Outcome", &round.outcome);

        formatter.add_field_aligned("Money Change", &format_money(round.money_change));
        formatter.add_field_aligned("Money After", &round.money_after);

        println!("\n{}\n", formatter.build());
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
        let remaining = cards.len() - 2;
        format!("{} +{remaining}", visible.join(" "))
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
