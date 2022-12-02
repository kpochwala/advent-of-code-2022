
enum Figure {
    Rock(String),
    Paper(String),
    Scissors(String),
}


fn string_to_figure(figure: String) -> Figure {
    let rock : Figure = Figure::Rock(String::from("A"));
    let paper : Figure = Figure::Paper(String::from("B"));
    let scissors : Figure = Figure::Scissors(String::from("C"));
    
    return rock;
}
struct Game {
    player: Figure,
    opponent: Figure,
    score: u32,
}

fn main() {
    println!("Hello, world!");
}
