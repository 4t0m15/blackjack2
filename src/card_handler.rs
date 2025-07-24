use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::hand_handler::{hand_value, card_art_index, draw};

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
    pub deck_index: i32
}

fn load_art_sections() -> HashMap<String, Vec<String>> {
    let mut art_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    art_path.push("src/otherart.txt");
    let content = fs::read_to_string(art_path).expect("Failed to read otherart.txt");
    let mut sections = HashMap::new();
    let mut current_section = String::new();
    let mut current_lines = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("// ---") {
            if !current_section.is_empty() {
                sections.insert(current_section.clone(), current_lines.clone());
            }
            current_section = line.trim().to_string();
            current_lines = Vec::new();
        } else {
            current_lines.push(line.to_string());
        }
    }
    if !current_section.is_empty() {
        sections.insert(current_section, current_lines);
    }
    sections
}

fn get_card_art() -> Vec<String> {
    let sections = load_art_sections();
    let art_lines = sections.get("// --- Card ASCII Art (from card_handler.rs) ---").expect("Card art section missing");
    let mut cards = Vec::new();
    let mut current = String::new();
    for line in art_lines {
        if line.trim().is_empty() && !current.is_empty() {
            cards.push(current.trim_end().to_string());
            current = String::new();
        } else {
            current.push_str(line);
            current.push('\n');
        }
    }
    if !current.trim().is_empty() {
        cards.push(current.trim_end().to_string());
    }
    cards
}

fn get_splash_screen() -> String {
    let sections = load_art_sections();
    let splash_lines = sections.get("// --- Splash Screen ASCII Art (from card_handler.rs and text_handler.rs) ---").expect("Splash screen section missing");
    splash_lines.join("\n")
}

fn get_message(key: &str, state: Option<&GameState>) -> String {
    let sections = load_art_sections();
    let msg_lines = sections.get("// --- Game Prompts and Messages (from card_handler.rs) ---").expect("Messages section missing");
    for line in msg_lines {
        if line.contains(key) {
            let mut msg = line.to_string();
            if let Some(s) = state {
                msg = msg.replace("{{money}}", &s.money.to_string())
                    .replace("{{gamesWon}}", &s.games_won.to_string())
                    .replace("{{gamesLost}}", &s.games_lost.to_string())
                    .replace("{{bet}}", &s.bet.to_string());
                if !s.dealer_cards.is_empty() {
                    msg = msg.replace("{{dealerCard}}", &s.dealer_cards[0]);
                }
                if !s.player_cards.is_empty() {
                    msg = msg.replace("{{playerCards}}", &s.player_cards.join(", "));
                }
                msg = msg.replace("{{playerTotal}}", &hand_value(&s.player_cards).to_string())
                    .replace("{{dealerTotal}}", &hand_value(&s.dealer_cards).to_string())
                    .replace("{{dealerCards}}", &s.dealer_cards.join(", "));
            }
            return msg;
        }
    }
    format!("[{}]", key)
}

pub fn start_blackjack() {
    let mut state = GameState {
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
            dealer_turn(&mut state);
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

fn create_deck() -> Vec<String> {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
    suits
        .iter()
        .flat_map(|s| ranks.iter().map(move |r| format!("{} {}", r, s)))
        .collect()
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

fn print_game_status(state: &GameState) {
    println!("You have {} coins", state.money);
    println!("Games won: {} | Games lost: {}", state.games_won, state.games_lost);
}

fn print_player_cards(state: &GameState) {
    let card_art = get_card_art();
    println!("{} {}", get_message("Your cards", Some(state)), state.player_cards.join(", "));
    println!("{}", get_message("Your total", Some(state)));

    // Collect the ASCII art for each card in the player's hand
    let card_arts: Vec<Vec<&str>> = state.player_cards
        .iter()
        .map(|card| card_art[card_art_index(card)].lines().collect())
        .collect();

    if card_arts.is_empty() {
        return;
    }

    // Print the ASCII art side by side
    for line_idx in 0..card_arts[0].len() {
        for card in &card_arts {
            print!("{} ", card[line_idx]);
        }
        println!();
    }
}

fn player_turn(state: &mut GameState) -> bool {
    loop {
        print!("Do you want to (h)it, (s)tand, or (d)ouble down? ");
        io::stdout().flush().ok();
        let c = read_char();
        if c == 'h' {
            let card = draw(state);
            state.player_cards.push(card.clone());
            state.player_card_count = state.player_cards.len() as i32;
            println!("You got: {}", card);
            print_player_cards(state);
            if hand_value(&state.player_cards) > 21 {
                println!("Bust! You went over 21!");
                state.games_lost += 1;
                return false;
            }
        } else if c == 's' {
            return true;
        } else if c == 'd' && state.money >= state.bet {
            state.money -= state.bet;
            state.bet *= 2;
            let card = draw(state);
            state.player_cards.push(card.clone());
            state.player_card_count = state.player_cards.len() as i32;
            println!("You got: {}", card);
            print_player_cards(state);
            if hand_value(&state.player_cards) > 21 {
                println!("Bust! You went over 21!");
                state.games_lost += 1;
                return false;
            }
            return true;
        } else {
            println!("Please type 'h', 's', or 'd'.");
        }
    }
}

fn dealer_turn(state: &mut GameState) {
    println!("Dealer's turn:");
    println!("Dealer's cards: [{}, Hidden]", state.dealer_cards[0]);
    while hand_value(&state.dealer_cards) < 17 {
        let card = draw(state);
        state.dealer_cards.push(card.clone());
        state.dealer_card_count = state.dealer_cards.len() as i32;
        println!("Dealer draws: {}", card);
    }
    println!(
        "Dealer's cards: {:?}",
        state.dealer_cards.iter().map(|c| c.as_str()).collect::<Vec<_>>()
    );
    let card_art = get_card_art();
    let card_arts: Vec<Vec<&str>> = state.dealer_cards
        .iter()
        .map(|card| card_art[card_art_index(card)].lines().collect())
        .collect();
    if !card_arts.is_empty() {
        for line_idx in 0..card_arts[0].len() {
            for card in &card_arts {
                print!("{} ", card[line_idx]);
            }
            println!();
        }
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

fn player_wins(state: &mut GameState) {
    println!("\x1b[1;32m{}\x1b[0m", get_message("You Win!", None));
    state.money += state.bet * 2;
    state.games_won += 1;
}
fn dealer_wins(state: &mut GameState) {
    println!("\x1b[1;31m{}\x1b[0m", get_message("Dealer Wins!", None));
    state.games_lost += 1;
}

fn read_char() -> char {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().chars().next().unwrap_or('\n')
}