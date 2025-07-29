use std::collections::VecDeque;

/// The starting amount of money the player has.
pub const STARTING_MONEY: i32 = 10;
#[allow(dead_code)]
/// The maximum number of cards a hand can have (for display/array sizing).
pub const MAX_HAND_SIZE: usize = 11;

/// ASCII art for each card rank, indexed as: A, 2, 3, ..., 10, J, Q, K
pub const CARD_ART: [&str; 13] = [
    "  _____\n |A .  |\n | /.\\ |\n |(_._)|\n |  |  |\n |____A|",
    "  _____\n |2    |\n |  ^  |\n |     |\n |  ^  |\n |____2|",
    "  _____\n |3    |\n | ^ ^ |\n |     |\n |  ^  |\n |____3|",
    "  _____\n |4    |\n | ^ ^ |\n |     |\n | ^ ^ |\n |____4|",
    "  _____\n |5    |\n | ^ ^ |\n |  ^  |\n | ^ ^ |\n |____5|",
    "  _____\n |6    |\n | ^ ^ |\n | ^ ^ |\n | ^ ^ |\n |____6|",
    "  _____\n |7    |\n | ^ ^ |\n |^ ^ ^|\n | ^ ^ |\n |____7|",
    "  _____\n |8    |\n |^ ^ ^|\n |^ ^ ^|\n |^ ^ ^|\n |____8|",
    "  _____\n |9    |\n |^ ^ ^|\n |^ ^ ^|\n |^ ^ ^|\n |____9|",
    "  _____\n |10 ^ |\n |^ ^ ^|\n |^ ^ ^|\n |^ ^ ^|\n |___10|",
    "  _____\n |J  ww|\n | ^ {)|\n |(.)%%|\n | |%%%|\n |_%%%>|",
    "  _____\n |Q  ww|\n | ^ {(|\n |(.)%%|\n | |%%%|\n |_%%%>|",
    "  _____\n |K  WW|\n | ^ {)|\n |(.)%%|\n | |%%%|\n |_%%%>|",
];

/// The main game state, tracking all mutable game data.
#[derive(Default)]
pub struct GameState {
    /// Player's current money.
    pub money: i32,
    /// Current bet for the round.
    pub bet: i32,
    /// Number of games won.
    pub games_won: i32,
    /// Number of games lost.
    pub games_lost: i32,
    /// Player's current hand of cards.
    pub player_hand: Vec<String>,
    /// Dealer's current hand of cards.
    pub dealer_hand: Vec<String>,
    /// The deck of cards for the current round.
    pub deck: VecDeque<String>,
}
