use termion::{
    style,
    color,
};

pub fn print_red(s: &str) {
    println!("\t{}{}{}{}", style::Bold, color::Fg(color::Red), s, style::Reset);
}

pub fn print_green(s: &str) {
    println!("\t{}{}{}{}", style::Bold, color::Fg(color::Green), s, style::Reset);
}

pub fn print_line() {
    println!("{}{}", style::Bold, color::Fg(color::Yellow));
    for _ in 0..100 {
        print!("=");
    }
    print!("\n\n{}", style::Reset);
}
