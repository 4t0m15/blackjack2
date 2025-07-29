pub struct PrintAbout;
impl PrintAbout {
    pub fn doit(&self) {
        println!(
            "\
         About: A simple text-based blackjack game written in Rust.
         Work is CC0 and source code is found at: (https://github.com/4t0m15/blackjack2). \
         If you paid for it, you should request a refund.\
         Created by Arsen Martirosyan."
        );
    }
}
pub struct PrintHelp;
impl PrintHelp {
    pub fn show_controls(&self) {
        print!(
            "\
    Controls: \
        n - Game Start\
        a - About\
        h - Help (where you are now)\
        q - Quit"
        );
    }
}
