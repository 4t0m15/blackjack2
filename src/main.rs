//imputs
//use std::io;
use std::time::Duration;
mod art_handler;
mod card_handler;
mod enemy_ai_handler;
mod hand_handler;
mod main_menu;
mod player_handler;
mod text_handler;

struct BlackJack;
impl BlackJack {
    // fn test(&self) {
    //     print!(" ");
    // }
    fn run(&self) {
        loop {
            text_handler::print_menu();
            let input = text_handler::read_menu_input();
            match input.as_str() {
                "a" => {
                    let about = main_menu::PrintAbout;
                    about.doit();
                }
                "h" => {
                    let help = main_menu::PrintHelp;
                    help.show_controls();
                    help.show_instructions();
                }
                "n" => {
                    card_handler::start_blackjack();
                }
                "q" => {
                    std::process::exit(0);
                }
                _ => {
                    text_handler::print_invalid_option();
                }
            }
            let show_splash = text_handler::ShowSplash;
            show_splash.doit();
        }
    }
}

fn main() {
    let game = BlackJack;
    game.run();
}

fn delay() {
    std::thread::sleep(Duration::from_secs(2));
}
