pub struct PrintAbout;
impl PrintAbout {
    pub fn doit(&self) {
        println!("{}", crate::art_handler::get_about_text());
    }
}
pub struct PrintHelp;
impl PrintHelp {
    pub fn show_controls(&self) {
        println!("{}", crate::art_handler::get_help_text("controls"));
    }
    pub fn show_instructions(&self) {
        println!("{}", crate::art_handler::get_help_text("instructions"));
    }
}
