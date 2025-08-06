use crate::game_history_core::GameHistory;

impl GameHistory {
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
        println!("║                    RECENT GAMES (Last {recent_count})                      ║");
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
            "║                       GAME #{round_number:<3} DETAILS                        ║"
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

        println!("║ Money Change: {money_display:54} ║");
        println!("║ Money After: {:55} ║", round.money_after);
        println!("╚══════════════════════════════════════════════════════════════════╝\n");
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
