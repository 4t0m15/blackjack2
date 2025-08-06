use crate::art_handler::{get_message, get_splash_screen, print_game_status};
use crate::game_history::{GameHistory, GameOutcome, GameRound};
use chrono::Local;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::enemy_ai_handler;
use crate::player_handler::{hand_value, player_turn, player_wins, print_player_cards};

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

impl GameState {
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

fn create_deck() -> Vec<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
    ];
    suits
        .iter()
        .flat_map(|s| ranks.iter().map(move |r| format!("{} {}", r, s)))
        .collect()
}

pub fn start_blackjack() {
    let mut state = GameState::new();
    start_blackjack_with_state(&mut state);
}

pub fn start_blackjack_with_state(state: &mut GameState) {
    print_splash_screen();
    delay();
    loop {
        print!("{}\n", get_message("You have", Some(&state)));
        io::stdout().flush().ok();
        if state.money <= 0 {
            println!("\x1b[1;31m{}\x1b[0m", get_message("Game Over", None));
            print!("{} ", get_message("Do you want to (t)ry again", None));
            io::stdout().flush().ok();
            let c = read_char();
            if c == 't' {
                state.money = 10;
                state.games_won = 0;
                state.games_lost = 0;
                state.history = GameHistory::new();
                continue;
            } else {
                break;
            }
        }
        setup_new_round(state);
        print_game_status(&state);
        state.current_round_start_money = state.money;
        state.bet = get_bet(&state);
        if state.bet == -1 {
            break;
        }
        state.money -= state.bet;
        state.was_double_down = false;
        println!("{}", get_message("Dealer shows", Some(&state)));
        print_player_cards(&state);
        if player_turn(state) {
            enemy_ai_handler::dealer_turn(state);
            determine_winner(state);
        }
    }
}

fn print_splash_screen() {
    println!("{}", get_splash_screen());
    delay();
    println!("{}", get_message("loading...", None));
}

fn delay() {
    thread::sleep(Duration::from_secs(2));
}

fn setup_new_round(state: &mut GameState) {
    state.card_deck = create_deck();
    state.card_deck.shuffle(&mut thread_rng());
    state.player_cards.clear();
    state.dealer_cards.clear();
    state.player_cards.push(state.card_deck[0].clone());
    state.player_cards.push(state.card_deck[1].clone());
    state.dealer_cards.push(state.card_deck[2].clone());
    state.dealer_cards.push(state.card_deck[3].clone());
    state.deck_index = 4;
    state.player_card_count = state.player_cards.len() as i32;
    state.dealer_card_count = state.dealer_cards.len() as i32;
}

fn get_bet(state: &GameState) -> i32 {
    loop {
        print!("How many coins do you want to bet? (m to return to main menu): ");
        io::stdout().flush().ok();
        let mut line = String::new();
        io::stdin().read_line(&mut line).ok();

        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("m") {
            return -1; // Special value to indicate returning to menu
        }

        if let Ok(n) = trimmed.parse::<i32>() {
            if n > 0 && n <= state.money {
                return n;
            }
        }
        println!("Please bet between 1 and {} coins.", state.money);
    }
}

fn determine_winner(state: &mut GameState) {
    let p_total = hand_value(&state.player_cards);
    let d_total = hand_value(&state.dealer_cards);
    println!("Your total: {}", p_total);
    println!("Dealer's total: {}", d_total);
    use std::cmp::Ordering;

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
        outcome: outcome.clone(),
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
        eprintln!("Failed to save game history to stats.csv: {}", e);
    }
}

pub fn read_char() -> char {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().chars().next().unwrap_or('\n')
}
