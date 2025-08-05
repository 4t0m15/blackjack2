mod art_handler;
mod card_handler;
mod enemy_ai_handler;
mod main_menu;
mod menu_handling;
mod player_handler;
mod save_system;
mod text_handler;

use menu_handling::Menu;

struct BlackJack {
    menu: Menu,
}

//"no boilerplate in rust", yeah right!
impl BlackJack {
    fn new() -> Self {
        BlackJack { menu: Menu::new() }
    }

    fn run(&mut self) {
        let show_splash = text_handler::ShowSplash;
        show_splash.doit();

        self.menu.run_loop();
    }
}

fn main() {
    let mut game = BlackJack::new();
    game.run();
}