use tidest::render::*;

fn main() {
    let mut style = Style::default().italic(Some(true)).bold(Some(true));
    style.foreground(Some(Color::Red));
    println!("{}Hello, world!", style);
}
