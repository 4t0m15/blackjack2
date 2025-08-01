use crate::art_handler::{get_message, get_splash_screen, print_game_status};
use rand::seq::SliceRandom;
use rand::thread_rng;
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
    let mut state = GameState {
        card_deck: Vec::new(),
        player_cards: Vec::new(),
        dealer_cards: Vec::new(),
        money: 100,
        bet: 0,
        games_won: 0,
        games_lost: 0,
        player_card_count: 0,
        dealer_card_count: 0,
        deck_index: 0,
    };
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
                continue;
            } else {
                break;
            }
        }
        setup_new_round(&mut state);
        print_game_status(&state);
        state.bet = get_bet(&state);
        state.money -= state.bet;
        println!("{}", get_message("Dealer shows", Some(&state)));
        print_player_cards(&state);
        if player_turn(&mut state) {
            enemy_ai_handler::dealer_turn(&mut state);
            determine_winner(&mut state);
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
        print!("How many coins do you want to bet? ");
        io::stdout().flush().ok();
        let mut line = String::new();
        io::stdin().read_line(&mut line).ok();
        if let Ok(n) = line.trim().parse::<i32>() {
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
    match (p_total > 21, d_total > 21, p_total.cmp(&d_total)) {
        (_, true, _) => player_wins(state),
        (true, _, _) => dealer_wins(state),
        (_, _, Ordering::Greater) => player_wins(state),
        (_, _, Ordering::Less) => dealer_wins(state),
        _ => {
            println!("It's a tie!");
            state.money += state.bet;
        }
    }
}

fn dealer_wins(state: &mut GameState) {
    println!("\x1b[1;31m{}\x1b[0m", get_message("Dealer Wins!", None));
    state.games_lost += 1;
}

pub fn read_char() -> char {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().chars().next().unwrap_or('\n')
}
