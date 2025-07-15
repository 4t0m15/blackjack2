use std::time::Duration;
mod textHandler;

struct BlackJack;
impl BlackJack {
    fn test(&self) {
        print!(" ");
    }
}

fn main() {
    let bjack = BlackJack;
    bjack.test();
    print!(" ");
    delay();
    print!(" ");
    let showSplash = textHandler::showsplash;
    showSplash.doit();
}

fn delay() {
    std::thread::sleep(Duration::from_secs(5));
}
