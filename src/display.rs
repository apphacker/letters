use term_painter::ToStyle;
use term_painter::Color::*;
use letter;


pub fn letters(msg: &str, options: &super::Options) {
    let msg = msg.to_lowercase();
    let mut final_output = String::new();
    for char in msg.chars() {
        let result = letter::get(char, options);
        final_output.push_str(&result);
    }
    println!("{}", final_output);
}

pub fn print_log_message(msg: &str) {
    println!("  ↘️️    {}", Yellow.paint(msg));
}

pub fn print_debug_message(msg: &str) {
    println!("  ↗️️️️    {}", BrightMagenta.paint(msg));
}
