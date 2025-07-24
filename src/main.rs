//imputs
//use std::io;
use std::time::Duration;
mod card_handler;
mod main_menu;
mod text_handler;
mod hand_handler;

struct BlackJack;
impl BlackJack {
    // fn test(&self) {
    //     print!(" ");
    // }
    fn run(&self) {
        let mut input_string = String::new();
        let show_splash = text_handler::ShowSplash;
        delay();
        show_splash.doit();
        loop {
            println!("Choose an option: (a)bout, (n)ew game, (h)elp, (q)uit: ");
            input_string.clear();
            std::io::stdin()
                .read_line(&mut input_string)
                .expect("text input failed");
            let input = input_string.trim();
            if input == "a" {
                let about = main_menu::PrintAbout;
                about.doit();
            } else if input == "h" {
                let help = main_menu::PrintHelp;
                help.show_controls();
                help.show_instructions();
            } else if input == "n" {
                // Start new game using card_handler
                card_handler::start_blackjack();
            } else if input == "q" {
                std::process::exit(0);
            }
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
