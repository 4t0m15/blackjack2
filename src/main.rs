//imputs
use std::time::Duration;
use std::io;
mod text_handler;
mod card_handler;
mod main_menu;

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
    loop {
        let mut input_string = String::new();
        println!("Choose an option: (a)bout, (n)ew game, (h)elp: ");
        io::stdin()
            .read_line(&mut input_string)
            .expect("text input failed");
        let showSplash = text_handler::showsplash;
        showSplash.doit();
    }
}

fn delay() {
    std::thread::sleep(Duration::from_secs(5));
}
