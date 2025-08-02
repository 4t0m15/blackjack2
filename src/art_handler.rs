use crate::card_handler::GameState;
use crate::player_handler::hand_value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn load_art_sections() -> HashMap<String, Vec<String>> {
    let mut art_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    art_path.push("src/art.txt");
    let content = fs::read_to_string(art_path).expect("Failed to read art.txt file");
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

pub fn get_card_art() -> Vec<String> {
    let sections = load_art_sections();
    let art_lines = sections
        .get("// --- Card ASCII Art (from card_handler.rs) ---")
        .expect("Card art section missing");
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

pub fn get_splash_screen() -> String {
    let sections = load_art_sections();
    let splash_lines = sections
        .get("// --- Splash Screen ASCII Art (from card_handler.rs and text_handler.rs) ---")
        .expect("Splash screen section missing");
    splash_lines.join("\n")
}

pub fn get_message(key: &str, state: Option<&GameState>) -> String {
    let sections = load_art_sections();
    let msg_lines = sections
        .get("// --- Game Prompts and Messages (from card_handler.rs) ---")
        .expect("Messages section missing");
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
    format!("[{}]", key)
}

pub fn print_game_status(state: &GameState) {
    println!("You have {} coins", state.money);
    println!(
        "Games won: {} | Games lost: {}",
        state.games_won, state.games_lost
    );
}
