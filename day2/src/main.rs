
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
enum Figure {
    Rock,
    Paper,
    Scissors,
}


fn string_to_figure(figure: String) -> Option<Figure> {
    match figure.as_str(){
        "A" => Some(Figure::Rock),
        "B" => Some(Figure::Paper),
        "C" => Some(Figure::Scissors),
        "X" => Some(Figure::Rock),
        "Y" => Some(Figure::Paper),
        "Z" => Some(Figure::Scissors),
        &_ => None,
    }
}
#[derive(Clone)]
struct Duel {
    player: Option<Figure>,
    opponent: Option<Figure>,
}

fn parse_duel(player : &str, opponent: &str) -> Option<Duel> {
    Some(Duel {
        player: string_to_figure(String::from(player)),
        opponent: string_to_figure(String::from(opponent)),
    })
}

enum DuelOutcome {
    Win,
    Tie,
    Loose,
}

fn settle_duel(duel: Duel) -> Option<DuelOutcome> {
    match duel.player? {
        Figure::Rock => {
            match duel.opponent? {
                Figure::Rock => Some(DuelOutcome::Tie),
                Figure::Paper => Some(DuelOutcome::Loose),
                Figure::Scissors => Some(DuelOutcome::Win),
            }
        },
        Figure::Paper => {
            match duel.opponent? {
                Figure::Rock => Some(DuelOutcome::Win),
                Figure::Paper => Some(DuelOutcome::Tie),
                Figure::Scissors => Some(DuelOutcome::Loose),
            }
        },
        Figure::Scissors => {
            match duel.opponent? {
                Figure::Rock => Some(DuelOutcome::Loose),
                Figure::Paper => Some(DuelOutcome::Win),
                Figure::Scissors => Some(DuelOutcome::Tie),
            }
        }
    }
}

fn score_duel(duel: Option<Duel>) -> Option<u32> {
    let mut score = match duel.clone()?.player? {
        Figure::Rock => 1,
        Figure::Paper => 2,
        Figure::Scissors => 3,
    };
    score += match settle_duel(duel.clone()?)? {
        DuelOutcome::Loose => Some(0),
        DuelOutcome::Tie => Some(3),
        DuelOutcome::Win => Some(6),
    }.unwrap_or(0);
    Some(score)
}

fn parse_line(line: String) -> u32 {
    // println!("Line: {}", line);
    let split = line.split(" ").collect::<Vec<&str>>();
    let duel = parse_duel(split[1], split[0]);
    
    // println!("Opponent: {}, player: {}", split[0], split[1]);
    let score = score_duel(duel).unwrap_or(0);
    // println!("Score: {}", score);
    // println!("");
    score
}
fn main() -> io::Result<()> {

    let file = File::open("src/input");
    let reader = io::BufReader::new(file?).lines();
    let mut final_score = 0;
    for line in reader{
        match line {
            Result::Ok(value) => {
                final_score += parse_line(value);
            }
            Result::Err(_err) => {
                continue;
            }
        }
    }

    println!("Final Score: {}", final_score);
    Ok(())
}
