//imputs
use std::time::Duration;
use std::io;
mod text_handler;
mod card_handler;

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
        println!("select a choice:");
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
