mod card_handler;
mod main_game_logic;
mod main_menu;
mod text_handler;
mod variables;

use crate::main_menu::{PrintAbout, PrintHelp};
use std::io::{self, Write};
use variables::GameState;

fn main() {
    let mut state = GameState {
        money: variables::STARTING_MONEY,
        ..Default::default()
    };

    text_handler::print_splash();
    delay();

    let about = PrintAbout;
    let help = PrintHelp;
    loop {
        println!("Choose an option: (a)bout, (n)ew game, (h)elp: ");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "a" => about.doit(),
            "h" => help.show_controls(),
            "n" => start_new_game(&mut state),
            "q" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please choose again."),
        }
    }
}

fn start_new_game(state: &mut GameState) {
    text_handler::print_welcome();
    while state.money > 0 {
        main_game_logic::setup_new_round(state);
        println!("You have {} coins", state.money);
        println!(
            "Games won: {} | Games lost: {}",
            state.games_won, state.games_lost
        );
        state.bet = main_game_logic::get_bet(state);
        state.money -= state.bet;
        println!("Dealer shows: [{} , Hidden]", state.dealer_hand[0]);
        main_game_logic::print_player_cards(&state.player_hand);

        if main_game_logic::player_turn(state) {
            main_game_logic::dealer_turn(state);
            main_game_logic::determine_winner(state);
        }
    }
}

fn delay() {
    std::thread::sleep(std::time::Duration::from_secs(2));
}
