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
//ayo, so i proceed to have a joo koo, everybody shaka lakin on my joo koo, pink thumbs, pink buns and a yoo hoo
//aye, young metro want some smores nigga!
//kanye graduation complete lyrics: Good morning oooooh ooooh oooh good morning ooooh oooh oooh good morning, ooooh oooh ooooh wake mr west mr fresh omg he so coress (I forgot this part) fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien yoooo wtf.... LOLOLOLOLO white america, assasinate my characta...
//ayo, so i proceed to have a joo koo, everybody shaka lakin on my joo koo, pink thumbs, pink buns and a yoo hoo
//aye, young metro want some smores nigga!
//kanye graduation complete lyrics: Good morning oooooh ooooh oooh good morning ooooh oooh oooh good morning, ooooh oooh ooooh wake mr west mr fresh omg he so coress (I forgot this part) fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien fien yoooo wtf.... LOLOLOLOLO white america, assasinate my characta...