use crate::card_handler::{create_deck, get_hand_value};
use crate::variables::GameState;
use std::io::{self, Write};

pub fn setup_new_round(state: &mut GameState) {
    state.deck = create_deck();
    state.player_hand.clear();
    state.dealer_hand.clear();
    // Deal two cards to player and dealer
    state.player_hand.push(state.deck.pop_front().unwrap());
    state.player_hand.push(state.deck.pop_front().unwrap());
    state.dealer_hand.push(state.deck.pop_front().unwrap());
    state.dealer_hand.push(state.deck.pop_front().unwrap());
}

pub fn get_bet(state: &GameState) -> i32 {
    loop {
        print!("How many coins do you want to bet? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(bet) = input.trim().parse::<i32>() {
            if bet > 0 && bet <= state.money {
                return bet;
            }
        }
        println!("Please bet between 1 and {} coins.", state.money);
    }
}

pub fn player_turn(state: &mut GameState) -> bool {
    loop {
        print!("Do you want to (h)it, (s)tand, or (d)ouble down? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().chars().next().unwrap_or(' ');
        match choice {
            'h' => {
                if let Some(card) = state.deck.pop_front() {
                    state.player_hand.push(card);
                    println!("You got: {}", state.player_hand.last().unwrap());
                    print_player_cards(&state.player_hand);
                    if get_hand_value(&state.player_hand) > 21 {
                        println!("Bust! You went over 21!");
                        state.games_lost += 1;
                        return false;
                    }
                }
            }
            's' => return true,
            'd' => {
                if state.money >= state.bet {
                    state.money -= state.bet;
                    state.bet *= 2;
                    if let Some(card) = state.deck.pop_front() {
                        state.player_hand.push(card);
                        println!("You got: {}", state.player_hand.last().unwrap());
                        print_player_cards(&state.player_hand);
                        if get_hand_value(&state.player_hand) > 21 {
                            println!("Bust! You went over 21!");
                            state.games_lost += 1;
                            return false;
                        }
                    }
                    return true;
                } else {
                    println!("Not enough money to double down.");
                }
            }
            _ => println!("Please type 'h' for hit, 's' for stand, or 'd' for double down."),
        }
    }
}

pub fn dealer_turn(state: &mut GameState) {
    println!("Dealer's turn:");
    println!("Dealer's cards: [{}, Hidden]", state.dealer_hand[0]);
    while get_hand_value(&state.dealer_hand) < 17 {
        if let Some(card) = state.deck.pop_front() {
            state.dealer_hand.push(card.clone());
            println!("Dealer draws: {}", card);
        }
    }
    println!("Dealer's cards: {:?}", state.dealer_hand);
}

pub fn determine_winner(state: &mut GameState) {
    let player_total = get_hand_value(&state.player_hand);
    let dealer_total = get_hand_value(&state.dealer_hand);
    println!("Your total: {}", player_total);
    println!("Dealer's total: {}", dealer_total);
    if dealer_total > 21 {
        println!("Dealer busts! You win!");
        state.money += state.bet * 2;
        state.games_won += 1;
    } else if player_total > dealer_total {
        println!("You win!");
        state.money += state.bet * 2;
        state.games_won += 1;
    } else if player_total < dealer_total {
        println!("Dealer wins!");
        state.games_lost += 1;
    } else {
        println!("It's a tie!");
        state.money += state.bet;
    }
}

pub fn print_player_cards(hand: &[String]) {
    print!("Your cards: ");
    for card in hand {
        print!("{} ", card);
    }
    println!("\nYour total: {}", get_hand_value(hand));
}
