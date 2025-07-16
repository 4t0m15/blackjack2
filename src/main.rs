//imputs
use std::io;
use std::time::Duration;
mod card_handler;
mod main_menu;
mod text_handler;

struct BlackJack;
impl BlackJack {
    fn test(&self) {
        print!(" ");
    }
}

fn main() {
    // let bjack = BlackJack;
    // bjack.test();
    // print!(" ");
    // delay();
    // print!(" ");
    let mut input_string = String::new();
    loop {
        println!("Choose an option: (a)bout, (n)ew game, (h)elp: ");
        io::stdin()
            .read_line(&mut input_string)
            .expect("text input failed");
        let showSplash = text_handler::showsplash;
        delay();
        if input_string == "a" {
            let about = main_menu::printAbout;
            about.doit();
        }
        showSplash.doit();
    }
}

fn delay() {
    std::thread::sleep(Duration::from_secs(2));
}
