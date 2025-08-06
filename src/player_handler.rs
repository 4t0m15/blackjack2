use crate::art_handler::{get_card_art, get_error_message};
use crate::game_loop::read_char;
use crate::game_state::GameState;
use std::io::{self, Write};

pub fn hand_value(hand: &[String]) -> i32 {
    let mut total = 0;
    let mut aces = 0;
    for card in hand {
        let rank = match card.split_whitespace().next() {
            Some(rank) => rank,
            None => {
                eprintln!("{}", get_error_message("Invalid card format"));
                continue;
            }
        };
        match rank {
            "A" => {
                total += 11;
                aces += 1;
            }
            "K" | "Q" | "J" => total += 10,
            _ => total += rank.parse::<i32>().unwrap_or(0),
        }
    }
    while total > 21 && aces > 0 {
        total -= 10;
        aces -= 1;
    }
    total
}

pub fn card_art_index(card: &str) -> usize {
    match card.split_whitespace().next() {
        Some("A") => 0,
        Some("2") => 1,
        Some("3") => 2,
        Some("4") => 3,
        Some("5") => 4,
        Some("6") => 5,
        Some("7") => 6,
        Some("8") => 7,
        Some("9") => 8,
        Some("10") => 9,
        Some("J") => 10,
        Some("Q") => 11,
        Some("K") => 12,
        _ => {
            eprintln!("{}", get_error_message("Unknown card rank"));
            0
        }
    }
}

pub fn draw(state: &mut GameState) -> String {
    let card = state.card_deck[state.deck_index as usize].clone();
    state.deck_index += 1;
    card
}

pub fn print_player_cards(state: &GameState) {
    println!("Your total: {}", hand_value(&state.player_cards));
    let card_art = get_card_art();
    let card_arts: Vec<Vec<&str>> = state
        .player_cards
        .iter()
        .map(|card| card_art[card_art_index(card)].lines().collect())
        .collect();
    if card_arts.is_empty() {
        return;
    }
    for line_idx in 0..card_arts[0].len() {
        for card in &card_arts {
            print!("{} ", card[line_idx]);
        }
        println!();
    }
}

fn print_hand_cards(hand: &[String], hand_name: &str) {
    println!("{} total: {}", hand_name, hand_value(hand));
    let card_art = get_card_art();
    let card_arts: Vec<Vec<&str>> = hand
        .iter()
        .map(|card| card_art[card_art_index(card)].lines().collect())
        .collect();
    if card_arts.is_empty() {
        return;
    }
    for line_idx in 0..card_arts[0].len() {
        for card in &card_arts {
            print!("{} ", card[line_idx]);
        }
        println!();
    }
}

fn can_split(state: &GameState) -> bool {
    if state.player_card_count != 2 || state.money < state.bet {
        return false;
    }

    let card1_rank = state.player_cards[0]
        .split_whitespace()
        .next()
        .unwrap_or("");
    let card2_rank = state.player_cards[1]
        .split_whitespace()
        .next()
        .unwrap_or("");

    // Allow splitting on same rank (including 10, J, Q, K all being 10-value cards)
    if card1_rank == card2_rank {
        return true;
    }

    // Allow splitting 10-value cards
    let is_ten_value = |rank: &str| matches!(rank, "10" | "J" | "Q" | "K");
    is_ten_value(card1_rank) && is_ten_value(card2_rank)
}

fn play_split_hand(hand: &mut Vec<String>, hand_name: &str, state: &mut GameState) -> bool {
    println!("\n--- Playing {hand_name} ---");

    // Draw one card for this hand
    let card = draw(state);
    hand.push(card.clone());
    println!("Drew: {card}");
    print_hand_cards(hand, hand_name);

    // Check for blackjack (21 with 2 cards)
    if hand.len() == 2 && hand_value(hand) == 21 {
        println!("Blackjack on {hand_name}!");
        return false; // No bust, stand automatically
    }

    // Play this hand
    loop {
        print!("Choose action for {hand_name}: (h)it, (s)tand: ");
        io::stdout().flush().ok();

        let action = read_char();
        match action {
            'h' => {
                let card = draw(state);
                hand.push(card.clone());
                println!("You got: {card}");
                print_hand_cards(hand, hand_name);

                if hand_value(hand) > 21 {
                    println!("{hand_name} busted!");
                    return true; // Busted
                }
            }
            's' => return false, // Stand, no bust
            _ => {
                println!("Invalid action, please choose again.");
            }
        }
    }
}

pub fn player_turn(state: &mut GameState) -> bool {
    loop {
        print!("Choose an action: (h)it, (s)tand");
        if state.player_card_count == 2 && state.money >= state.bet {
            print!(", (d)ouble down");
        }
        if can_split(state) {
            print!(", (p)split");
        }
        if state.player_card_count == 2 {
            print!(", s(u)rrender");
        }
        print!(": ");
        io::stdout().flush().ok();

        let action = read_char();
        match action {
            'h' => {
                let card = draw(state);
                state.player_cards.push(card.clone());
                state.player_card_count = state.player_cards.len() as i32;
                println!("You got: {card}");
                print_player_cards(state);

                if hand_value(&state.player_cards) > 21 {
                    println!("You busted! Dealer wins.");
                    state.games_lost += 1;
                    return false;
                }
            }
            's' => return true,
            'd' if state.player_card_count == 2 && state.money >= state.bet => {
                state.money -= state.bet;
                state.bet *= 2;
                state.was_double_down = true;
                let card = draw(state);
                state.player_cards.push(card.clone());
                state.player_card_count = state.player_cards.len() as i32;
                println!("You doubled down and drew: {card}");
                print_player_cards(state);

                if hand_value(&state.player_cards) > 21 {
                    println!("You busted! Dealer wins.");
                    state.games_lost += 1;
                    return false;
                }
                return true;
            }
            'd' if state.money < state.bet => {
                println!("Not enough money to double down!");
            }
            'p' if can_split(state) => {
                // Deduct additional bet for second hand
                state.money -= state.bet;

                // Split the cards
                let mut hand1 = vec![state.player_cards[0].clone()];
                let mut hand2 = vec![state.player_cards[1].clone()];

                println!("Splitting your hand!");

                // Play first hand
                let hand1_busted = play_split_hand(&mut hand1, "Hand 1", state);

                // Play second hand
                let hand2_busted = play_split_hand(&mut hand2, "Hand 2", state);

                // Determine outcome
                if hand1_busted && hand2_busted {
                    println!("Both hands busted! Dealer wins.");
                    state.games_lost += 1;
                    return false;
                } else if hand1_busted {
                    println!("Hand 1 busted, but Hand 2 is still alive.");
                    // Use hand2 as the main hand for dealer comparison
                    state.player_cards = hand2;
                    state.player_card_count = state.player_cards.len() as i32;
                    return true;
                } else if hand2_busted {
                    println!("Hand 2 busted, but Hand 1 is still alive.");
                    // Use hand1 as the main hand for dealer comparison
                    state.player_cards = hand1;
                    state.player_card_count = state.player_cards.len() as i32;
                    return true;
                } else {
                    // Both hands alive - use the better one for comparison
                    let hand1_total = hand_value(&hand1);
                    let hand2_total = hand_value(&hand2);

                    if hand1_total >= hand2_total {
                        println!("Using Hand 1 (total: {hand1_total}) for dealer comparison.");
                        state.player_cards = hand1;
                    } else {
                        println!("Using Hand 2 (total: {hand2_total}) for dealer comparison.");
                        state.player_cards = hand2;
                    }
                    state.player_card_count = state.player_cards.len() as i32;
                    return true;
                }
            }
            'p' if !can_split(state) => {
                if state.player_card_count != 2 {
                    println!("Can only split with exactly 2 cards!");
                } else if state.money < state.bet {
                    println!("Not enough money to split!");
                } else {
                    println!("Cannot split - cards must be same rank!");
                }
            }
            'u' if state.player_card_count == 2 => {
                println!("You surrendered. Half your bet is returned.");
                state.money += state.bet / 2;
                state.games_lost += 1;
                return false;
            }
            'u' if state.player_card_count != 2 => {
                println!("Can only surrender with your initial 2 cards!");
            }
            _ => {
                println!("Invalid action, please choose again.");
            }
        }
    }
}

pub fn player_wins(state: &mut GameState) {
    use crate::art_handler::get_message;
    println!("\x1b[1;32m{}\x1b[0m", get_message("You Win!", None));
    state.money += state.bet * 2;
    state.games_won += 1;
}
