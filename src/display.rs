use term_painter::ToStyle;
use term_painter::Color::*;


pub fn letters(msg: &str, options: &super::Options) {
    println!("{}", msg);
}

pub fn print_log_message(msg: &str) {
    println!("  ↘️️    {}", Yellow.paint(msg));
}

pub fn print_debug_message(msg: &str) {
    println!("  ↗️️️️    {}", BrightMagenta.paint(msg));
}
