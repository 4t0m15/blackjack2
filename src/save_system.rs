use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const SAVE_FILE: &str = "blackjack_save.json";
const DEFAULT_STARTING_MONEY: i32 = 100;

pub const STARTING_MONEY: i32 = DEFAULT_STARTING_MONEY;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SaveData {
    pub money: i32,
    pub games_won: i32,
    pub games_lost: i32,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            money: DEFAULT_STARTING_MONEY,
            games_won: 0,
            games_lost: 0,
        }
    }
}

pub fn load_save_data() -> SaveData {
    if Path::new(SAVE_FILE).exists() {
        match fs::read_to_string(SAVE_FILE) {
            Ok(content) => match serde_json::from_str::<SaveData>(&content) {
                Ok(save_data) => {
                    println!(
                        "✓ Loaded previous game data: {} coins, {} wins, {} losses",
                        save_data.money, save_data.games_won, save_data.games_lost
                    );
                    save_data
                }
                Err(e) => {
                    eprintln!("⚠ Warning: Could not parse save file ({e}). Starting fresh.");
                    SaveData::default()
                }
            },
            Err(e) => {
                eprintln!("⚠ Warning: Could not read save file ({e}). Starting fresh.");
                SaveData::default()
            }
        }
    } else {
        println!("No previous save found. Starting new game!");
        SaveData::default()
    }
}

pub fn save_game_data(save_data: &SaveData) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(save_data)?;
    fs::write(SAVE_FILE, json_data)?;
    println!("✓ Game progress saved!");
    Ok(())
}

pub fn auto_save(save_data: &SaveData) {
    if let Err(e) = save_game_data(save_data) {
        eprintln!("⚠ Warning: Could not save game data: {e}");
    }
}

pub fn create_save_data(money: i32, games_won: i32, games_lost: i32) -> SaveData {
    SaveData {
        money,
        games_won,
        games_lost,
    }
}
