pub struct ShowSplash;
impl ShowSplash {
    pub fn doit(&self) {
        println!("{}", crate::art_handler::get_splash_screen());
    }
}

pub fn print_menu() {
    println!("Choose an option: (a)bout, (n)ew game, (h)elp, (q)uit: ");
}

pub fn read_menu_input() -> String {
    use std::io::{self, Write};
    let mut input_string = String::new();
    io::stdout().flush().ok();
    io::stdin()
        .read_line(&mut input_string)
        .expect("text input failed");
    input_string.trim().to_string()
}

pub fn print_invalid_option() {
    println!("Invalid option. Please try again.");
}
