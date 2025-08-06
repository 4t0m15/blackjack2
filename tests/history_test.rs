#[cfg(test)]
mod tests {
    use blackjack2::game_history::{GameHistory, GameOutcome, GameRound};
    use chrono::Local;

    #[test]
    fn test_game_history_creation() {
        let history = GameHistory::new();
        assert_eq!(history.total_games_played, 0);
        assert_eq!(history.total_wins, 0);
        assert_eq!(history.total_losses, 0);
        assert_eq!(history.total_ties, 0);
        assert_eq!(history.get_win_rate(), 0.0);
        assert_eq!(history.get_net_profit(), 0);
    }

    #[test]
    fn test_add_winning_round() {
        let mut history = GameHistory::new();

        let round = GameRound {
            round_number: 1,
            timestamp: Local::now(),
            bet_amount: 10,
            player_cards: vec!["K Hearts".to_string(), "A Spades".to_string()],
            dealer_cards: vec!["10 Clubs".to_string(), "8 Diamonds".to_string()],
            player_total: 21,
            dealer_total: 18,
            outcome: GameOutcome::PlayerWin,
            money_change: 20,
            money_after: 30,
            was_double_down: false,
            player_busted: false,
            dealer_busted: false,
        };

        history.add_round(round);

        assert_eq!(history.total_games_played, 1);
        assert_eq!(history.total_wins, 1);
        assert_eq!(history.total_losses, 0);
        assert_eq!(history.get_win_rate(), 100.0);
        assert_eq!(history.total_money_won, 20);
        assert_eq!(history.biggest_win, 20);
    }

    #[test]
    fn test_add_losing_round() {
        let mut history = GameHistory::new();

        let round = GameRound {
            round_number: 1,
            timestamp: Local::now(),
            bet_amount: 5,
            player_cards: vec![
                "K Hearts".to_string(),
                "9 Spades".to_string(),
                "7 Clubs".to_string(),
            ],
            dealer_cards: vec!["10 Diamonds".to_string(), "A Hearts".to_string()],
            player_total: 26,
            dealer_total: 21,
            outcome: GameOutcome::PlayerBust,
            money_change: -5,
            money_after: 5,
            was_double_down: false,
            player_busted: true,
            dealer_busted: false,
        };

        history.add_round(round);

        assert_eq!(history.total_games_played, 1);
        assert_eq!(history.total_wins, 0);
        assert_eq!(history.total_losses, 1);
        assert_eq!(history.get_win_rate(), 0.0);
        assert_eq!(history.total_money_lost, 5);
        assert_eq!(history.biggest_loss, 5);
    }

    #[test]
    fn test_tie_round() {
        let mut history = GameHistory::new();

        let round = GameRound {
            round_number: 1,
            timestamp: Local::now(),
            bet_amount: 10,
            player_cards: vec!["K Hearts".to_string(), "Q Spades".to_string()],
            dealer_cards: vec!["10 Clubs".to_string(), "J Diamonds".to_string()],
            player_total: 20,
            dealer_total: 20,
            outcome: GameOutcome::Tie,
            money_change: 0,
            money_after: 10,
            was_double_down: false,
            player_busted: false,
            dealer_busted: false,
        };

        history.add_round(round);

        assert_eq!(history.total_games_played, 1);
        assert_eq!(history.total_wins, 0);
        assert_eq!(history.total_losses, 0);
        assert_eq!(history.total_ties, 1);
        assert_eq!(history.get_win_rate(), 0.0);
        assert_eq!(history.get_net_profit(), 0);
    }

    #[test]
    fn test_multiple_rounds_statistics() {
        let mut history = GameHistory::new();

        // Add 3 wins
        for i in 1..=3 {
            let round = GameRound {
                round_number: i,
                timestamp: Local::now(),
                bet_amount: 10,
                player_cards: vec!["K Hearts".to_string(), "A Spades".to_string()],
                dealer_cards: vec!["10 Clubs".to_string(), "8 Diamonds".to_string()],
                player_total: 21,
                dealer_total: 18,
                outcome: GameOutcome::PlayerWin,
                money_change: 20,
                money_after: 10 + (i as i32 * 20),
                was_double_down: false,
                player_busted: false,
                dealer_busted: false,
            };
            history.add_round(round);
        }

        // Add 1 loss
        let round = GameRound {
            round_number: 4,
            timestamp: Local::now(),
            bet_amount: 10,
            player_cards: vec![
                "K Hearts".to_string(),
                "9 Spades".to_string(),
                "7 Clubs".to_string(),
            ],
            dealer_cards: vec!["10 Diamonds".to_string(), "A Hearts".to_string()],
            player_total: 26,
            dealer_total: 21,
            outcome: GameOutcome::PlayerBust,
            money_change: -10,
            money_after: 60,
            was_double_down: false,
            player_busted: true,
            dealer_busted: false,
        };
        history.add_round(round);

        // Add 1 tie
        let round = GameRound {
            round_number: 5,
            timestamp: Local::now(),
            bet_amount: 10,
            player_cards: vec!["K Hearts".to_string(), "Q Spades".to_string()],
            dealer_cards: vec!["10 Clubs".to_string(), "J Diamonds".to_string()],
            player_total: 20,
            dealer_total: 20,
            outcome: GameOutcome::Tie,
            money_change: 0,
            money_after: 60,
            was_double_down: false,
            player_busted: false,
            dealer_busted: false,
        };
        history.add_round(round);

        assert_eq!(history.total_games_played, 5);
        assert_eq!(history.total_wins, 3);
        assert_eq!(history.total_losses, 1);
        assert_eq!(history.total_ties, 1);
        assert_eq!(history.get_win_rate(), 60.0);
        assert_eq!(history.total_money_won, 60);
        assert_eq!(history.total_money_lost, 10);
        assert_eq!(history.get_net_profit(), 50);
    }

    #[test]
    fn test_double_down_tracking() {
        let mut history = GameHistory::new();

        let round = GameRound {
            round_number: 1,
            timestamp: Local::now(),
            bet_amount: 20, // doubled from 10
            player_cards: vec![
                "5 Hearts".to_string(),
                "6 Spades".to_string(),
                "9 Clubs".to_string(),
            ],
            dealer_cards: vec!["10 Diamonds".to_string(), "8 Hearts".to_string()],
            player_total: 20,
            dealer_total: 18,
            outcome: GameOutcome::PlayerWin,
            money_change: 40, // double bet payout
            money_after: 50,
            was_double_down: true,
            player_busted: false,
            dealer_busted: false,
        };

        history.add_round(round);

        assert_eq!(history.rounds.len(), 1);
        assert!(history.rounds[0].was_double_down);
        assert_eq!(history.rounds[0].bet_amount, 20);
        assert_eq!(history.biggest_win, 40);
    }

    #[test]
    fn test_csv_export() {
        let mut history = GameHistory::new();

        let round = GameRound {
            round_number: 1,
            timestamp: Local::now(),
            bet_amount: 10,
            player_cards: vec!["K Hearts".to_string(), "A Spades".to_string()],
            dealer_cards: vec!["10 Clubs".to_string(), "8 Diamonds".to_string()],
            player_total: 21,
            dealer_total: 18,
            outcome: GameOutcome::PlayerWin,
            money_change: 20,
            money_after: 30,
            was_double_down: false,
            player_busted: false,
            dealer_busted: false,
        };

        history.add_round(round);

        let csv = history.export_to_csv();

        // Check CSV header
        assert!(csv.contains("Round,Timestamp,Bet,PlayerCards,DealerCards"));
        // Check CSV data
        assert!(csv.contains("Player Win"));
        assert!(csv.contains("K Hearts | A Spades"));
        assert!(csv.contains("10 Clubs | 8 Diamonds"));
    }
}
