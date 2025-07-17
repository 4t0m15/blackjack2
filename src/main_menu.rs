pub struct printAbout;
impl printAbout {
    pub fn doit(&self) {
        println!("\
         About: A simple text-based blackjack game written in Rust.
         Work is CC0 and source code is found at: (https://github.com/4t0m15/blackjack2). \
         If you paid for it, you should request a refund.\
         Created by Arsen Martirosyan.");
    }
}
pub struct printHelp;
impl printHelp {
    pub fn show_controls(&self) {
        print!("\
    Controls: \
        n - Game Start\
        a - About\
        h - Help (where you are now)\
        q - Quit");
    }
    pub fn show_instructions(&self) {
        print!(" How to play: \
        THE PLAYER is served a card by a DEALER.\
        THE PLAYER can choose to HIT and get a card.\
        Although this can lead the player to BUST.\
        A BUST is when THE PLAYER'S cards go over 21.\
        The DEALER will also be served a card.\
        The idea is to have more than the DEALER, but not BUST.");
    }
}