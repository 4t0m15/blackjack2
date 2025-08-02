use crate::card_handler;
use crate::main_menu;
use crate::text_handler;

#[derive(Copy)]
pub enum MenuAction {
    About,
    Help,
    Guide,
    NewGame,
    Quit,
    None,
}

pub struct Menu
fn handle_action(action: MenuAction) {
    match action {
        MenuAction::About => printAbout(),
        MenuAction::Help => println!("Help"),
        MenuAction::Guide => println!("Guide"),
        MenuAction::NewGame => println!("New Game"),
        MenuAction::Quit => println!("Quit"),
        MenuAction::None => println!("None"),
    }
}
