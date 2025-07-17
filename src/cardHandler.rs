struct GameState {
    cardDeck: Vec<String>,
    playerCards: Vec<String>,
    dealerCards: Vec<String>,
    money: i32,
    bet: i32,
    gamesWon: i32,
    gamesLost: i32,
    playerCardCount: i32,
    dealerCardCount: i32,
    deckIndex: i32
}
//Card art as an array
static cardArt: [&str; 14] = [
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
        "  _____\n |J  ww|\n | ^ {)|\n |(.)%%|\n | |%%%|\n |_%%%>|\n",
        "  _____\n |Q  ww|\n | ^ {(|\n |(.)%%|\n | |%%%|\n |_%%%>|\n",
        "  _____\n |K  WW|\n | ^ {)|\n |(.)%%|\n | |%%%|\n |_%%%>|\n",
        "  _____\n |A ^  |\n | / \\ |\n | \\ / |\n |  .  |\n |____A|"
];