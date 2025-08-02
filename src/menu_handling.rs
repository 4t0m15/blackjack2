use crate::card_handler;
use crate::main_menu;
use crate::text_handler;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MenuAction {
    About,
    Help,
    Guide,
    NewGame,
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
            let input = text_handler::read_menu_input();
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
                card_handler::start_blackjack();
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
