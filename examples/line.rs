use drawille_nostd::*;

fn main() {
    let mut canvas = Canvas::new(100, 100);

    canvas.line(10, 15, 80, 60);

    let frame = canvas.frame().replace(' ', "\u{2800}");

    println!("{frame}");
}

