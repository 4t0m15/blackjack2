use crate::game_loop;
use crate::main_menu;
use crate::text_handler;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MenuAction {
    About,
    Help,
    Guide,
    NewGame,
    History,
    Quit,
    Invalid,
}

pub struct Menu {
    current_action: MenuAction,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            current_action: MenuAction::Invalid,
        }
    }

    pub fn set_action(&mut self, action: MenuAction) {
        self.current_action = action;
    }

    pub fn execute(&self) {
        self.current_action.execute();
    }

    pub fn run_loop(&mut self) {
        loop {
            text_handler::print_menu();
            let input = match text_handler::read_menu_input() {
                Ok(input) => input,
                Err(_) => break, // Exit on input error
            };
            self.set_action(MenuAction::from_string(&input));

            if self.current_action == MenuAction::Quit {
                break;
            }

            self.execute();
        }
    }
}
impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuAction {
    pub fn from_string(input: &str) -> Self {
        match input.trim().to_lowercase().as_str() {
            "a" => MenuAction::About,
            "h" => MenuAction::Help,
            "g" => MenuAction::Guide,
            "n" => MenuAction::NewGame,
            "r" => MenuAction::History,
            "q" => MenuAction::Quit,
            _ => MenuAction::Invalid,
        }
    }

    pub fn execute(&self) {
        match self {
            MenuAction::About => {
                let about = main_menu::PrintAbout;
                about.doit();
            }
            MenuAction::Help => {
                let help = main_menu::PrintHelp;
                help.show_controls();
            }
            MenuAction::Guide => {
                let help = main_menu::PrintHelp;
                help.show_instructions();
            }
            MenuAction::NewGame => {
                game_loop::start_blackjack();
            }
            MenuAction::History => {
                println!("History functionality is handled by GameManager");
            }
            MenuAction::Quit => {
                std::process::exit(0);
            }
            MenuAction::Invalid => {
                text_handler::print_invalid_option();
            }
        }
    }
}
