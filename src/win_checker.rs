use crate::art_handler::get_message;
use crate::game_history::{GameOutcome, GameRound};
use crate::game_state::GameState;
use crate::player_handler::{hand_value, player_wins};
use chrono::Local;
use std::cmp::Ordering;
use std::fs;

pub fn determine_winner(state: &mut GameState) {
    let p_total = hand_value(&state.player_cards);
    let d_total = hand_value(&state.dealer_cards);
    println!("Your total: {p_total}");
    println!("Dealer's total: {d_total}");

    let outcome = match (p_total > 21, d_total > 21, p_total.cmp(&d_total)) {
        (_, true, _) => {
            player_wins(state);
            GameOutcome::DealerBust
        }
        (true, _, _) => {
            dealer_wins(state);
            GameOutcome::PlayerBust
        }
        (_, _, Ordering::Greater) => {
            player_wins(state);
            GameOutcome::PlayerWin
        }
        (_, _, Ordering::Less) => {
            dealer_wins(state);
            GameOutcome::DealerWin
        }
        _ => {
            println!("It's a tie!");
            state.money += state.bet;
            GameOutcome::Tie
        }
    };

    record_game_result(state, outcome);
}

fn dealer_wins(state: &mut GameState) {
    println!("\x1b[1;31m{}\x1b[0m", get_message("Dealer Wins!", None));
    state.games_lost += 1;
}

fn record_game_result(state: &mut GameState, outcome: GameOutcome) {
    let p_total = hand_value(&state.player_cards);
    let d_total = hand_value(&state.dealer_cards);
    let money_change = state.money - state.current_round_start_money;

    let round = GameRound {
        round_number: state.history.total_games_played + 1,
        timestamp: Local::now(),
        bet_amount: state.bet,
        player_cards: state.player_cards.clone(),
        dealer_cards: state.dealer_cards.clone(),
        player_total: p_total,
        dealer_total: d_total,
        outcome,
        money_change,
        money_after: state.money,
        was_double_down: state.was_double_down,
        player_busted: p_total > 21,
        dealer_busted: d_total > 21,
    };

    state.history.add_round(round);

    // Automatically save game history to CSV file
    let csv_content = state.history.export_to_csv();

    // Create path relative to the project directory
    let mut path = std::env::current_dir().unwrap_or_default();
    path.push("stats.csv");

    if let Err(e) = fs::write(&path, csv_content) {
        eprintln!("Failed to save game history to stats.csv: {e}");
    }

    // Save the current game state (money, wins, losses) to JSON file
    state.save_to_disk();
}
