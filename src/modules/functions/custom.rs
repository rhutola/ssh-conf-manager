/// Custom I/O modules
pub mod io_custom {
    use std::io::{ self, Write };

    /// Input function
    /// Type after Text
    pub fn input(text: &str) -> String {
        print!("{}", text);
        let mut input  = String::new();
        let __ = io::stdout().flush();
        std::io::stdin().read_line(&mut input).ok();

        return input.trim().to_string()
    }
}

/// Custom regular expression modules
pub mod regex_cutom {
    extern crate regex;
    use regex::Regex;

    /// Numerical character judgment
    /// True if only number
    pub fn is_number_string(target: String) -> bool {
        let number_regex = Regex::new(r"^[0-9]+$").unwrap();

        if number_regex.is_match(&target) {
            return true;
        }

        return false;
    }
}