use tidest::render::*;

fn main() {
    let style = Style::default().italic(Some(true)).bold(Some(true));
    println!("{}Hello, world!", style);
}
