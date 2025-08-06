use crate::game_state::GameState;
use crate::player_handler::hand_value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[must_use]
pub fn load_art_sections() -> HashMap<String, Vec<String>> {
    let mut art_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    art_path.push("src/art.txt");
    let content = match fs::read_to_string(&art_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Warning: Failed to read art.txt file: {e}. Using default values.");
            return HashMap::new();
        }
    };
    let mut sections = HashMap::new();
    let mut current_section = String::new();
    let mut current_lines = Vec::new();
    for line in content.lines() {
        if line.trim_start().starts_with("// ---") {
            if !current_section.is_empty() {
                sections.insert(current_section.clone(), current_lines.clone());
            }
            current_section = line.trim().to_string();
            current_lines = Vec::new();
        } else {
            current_lines.push(line.to_string());
        }
    }
    if !current_section.is_empty() {
        sections.insert(current_section, current_lines);
    }
    sections
}

#[must_use]
pub fn get_card_art() -> Vec<String> {
    let sections = load_art_sections();
    let Some(art_lines) = sections.get("// --- Card ASCII Art (from card_handler.rs) ---") else {
        eprintln!("Warning: Card art section missing. Using fallback.");
        return vec!["[CARD]".to_string(); 13];
    };
    let mut cards = Vec::new();
    let mut current = String::new();
    for line in art_lines {
        if line.trim().is_empty() && !current.is_empty() {
            cards.push(current.trim_end().to_string());
            current = String::new();
        } else {
            current.push_str(line);
            current.push('\n');
        }
    }
    if !current.trim().is_empty() {
        cards.push(current.trim_end().to_string());
    }
    cards
}

#[must_use]
pub fn get_splash_screen() -> String {
    let sections = load_art_sections();
    let Some(splash_lines) = sections
        .get("// --- Splash Screen ASCII Art (from card_handler.rs and text_handler.rs) ---")
    else {
        eprintln!("Warning: Splash screen section missing. Using fallback.");
        return "🃏 BLACKJACK 🃏\nWelcome to the game!".to_string();
    };
    splash_lines.join("\n")
}

#[must_use]
pub fn get_message(key: &str, state: Option<&GameState>) -> String {
    let sections = load_art_sections();
    let Some(msg_lines) =
        sections.get("// --- Game Prompts and Messages (from card_handler.rs) ---")
    else {
        eprintln!("Warning: Messages section missing. Using fallback.");
        return format!("[{key}]");
    };
    for line in msg_lines {
        if line.contains(key) {
            let mut msg = line.to_string();
            if let Some(s) = state {
                msg = msg
                    .replace("{{money}}", &s.money.to_string())
                    .replace("{{gamesWon}}", &s.games_won.to_string())
                    .replace("{{gamesLost}}", &s.games_lost.to_string())
                    .replace("{{bet}}", &s.bet.to_string());
                if !s.dealer_cards.is_empty() {
                    msg = msg.replace("{{dealerCard}}", &s.dealer_cards[0]);
                }
                if !s.player_cards.is_empty() {
                    msg = msg.replace("{{playerCards}}", &s.player_cards.join(", "));
                }
                msg = msg
                    .replace("{{playerTotal}}", &hand_value(&s.player_cards).to_string())
                    .replace("{{dealerTotal}}", &hand_value(&s.dealer_cards).to_string())
                    .replace("{{dealerCards}}", &s.dealer_cards.join(", "));
            }
            return msg;
        }
    }
    format!("[{key}]")
}

pub fn print_game_status(state: &GameState) {
    println!("\n{}", get_message("You have", Some(state)));
}

#[must_use]
pub fn get_menu_prompt() -> String {
    let sections = load_art_sections();
    let Some(menu_lines) = sections.get("// --- Main Menu Prompt (from main.rs) ---") else {
        eprintln!("Warning: Menu prompt section missing. Using fallback.");
        return "Choose an option: (a)bout, (n)ew game, (h)elp, (g)uide, (q)uit:".to_string();
    };
    menu_lines.join("\n")
}

#[must_use]
pub fn get_about_text() -> String {
    let sections = load_art_sections();
    let Some(about_lines) = sections.get("// --- About Text (from main_menu.rs) ---") else {
        eprintln!("Warning: About text section missing. Using fallback.");
        return "About: A simple text-based blackjack game written in Rust.".to_string();
    };
    about_lines.join("\n")
}

#[must_use]
pub fn get_help_text(help_type: &str) -> String {
    let sections = load_art_sections();
    let Some(help_lines) = sections.get("// --- Help Text (from main_menu.rs) ---") else {
        eprintln!("Warning: Help text section missing. Using fallback.");
        return "Help information not available.".to_string();
    };

    let mut result = Vec::new();
    let mut in_section = false;

    for line in help_lines {
        if line.starts_with("Controls:") {
            in_section = help_type == "controls";
            if in_section {
                result.push(line.clone());
            }
        } else if line.starts_with("Game Instructions:") {
            in_section = help_type == "instructions";
            if in_section {
                result.push(line.clone());
            }
        } else if in_section {
            result.push(line.clone());
        }
    }

    if result.is_empty() {
        format!("No {help_type} help available.")
    } else {
        result.join("\n")
    }
}

#[must_use]
pub fn get_error_message(error_key: &str) -> String {
    let sections = load_art_sections();
    let Some(error_lines) = sections.get("// --- Error Messages (from various files) ---") else {
        eprintln!("Warning: Error messages section missing. Using fallback.");
        return format!("Error: {error_key}");
    };

    for line in error_lines {
        if line.contains(error_key) {
            return line.clone();
        }
    }
    format!("Error: {error_key}")
}

#[must_use]
pub fn get_action_message(action_key: &str, state: Option<&GameState>) -> String {
    let sections = load_art_sections();
    let Some(action_lines) = sections.get("// --- Game Actions (from player_handler.rs) ---")
    else {
        eprintln!("Warning: Game actions section missing. Using fallback.");
        return format!("[{action_key}]");
    };

    for line in action_lines {
        if line.contains(action_key) {
            let mut msg = line.to_string();
            if let Some(s) = state {
                msg = msg
                    .replace("{{card}}", s.player_cards.last().unwrap_or(&String::new()))
                    .replace("{{bet}}", &s.bet.to_string());

                if msg.contains("{{payout}}") {
                    let payout = s.bet * 3;
                    msg = msg.replace("{{payout}}", &payout.to_string());
                }
            }
            return msg;
        }
    }
    format!("[{action_key}]")
}
