use std::fs::File;
use std::io::{self, BufRead};
use array_tool::vec::Intersect;
use std::collections::HashMap;

#[derive(Debug)]
struct Rucksack {
    compartment1: Vec<char>,
    compartment2: Vec<char>,
    common_in_both_compartments: Vec<char>,
}

impl Rucksack {
    fn from_pair(pair: (&str, &str)) -> Self {
        let (c1_str, c2_str) = pair;
        
        let c1: Vec<char> = c1_str.chars().collect();
        let c2: Vec<char> = c2_str.chars().collect();

        Self {
            compartment1: c1.clone(),
            compartment2: c2.clone(),
            common_in_both_compartments: c1.intersect(c2),
        }
    }
}

fn parse_to_slots(line: &String) -> (&str, &str) {
    let len = line.len();
    (&line[..len/2], &line[len/2..])
}

fn parse_line(line: &String) -> Rucksack {
    let (left, right) = parse_to_slots(line);
    Rucksack::from_pair(parse_to_slots(line))
}

fn create_scoring_map() -> HashMap<char, u32> {
    let mut score = 0;
    let mut map:HashMap<char, u32> = HashMap::new();

    for lowercase in 'a'..='z'{
        score += 1;
        map.insert(lowercase, score);
    }
    for uppercase in 'A'..='Z'{
        score += 1;
        map.insert(uppercase, score);
    }
    
    map
}

fn get_score(letter: char, map: &HashMap<char, u32>) -> u32 {
    map[&letter]
}

fn main() -> io::Result<()> {

    let file = File::open("src/input");
    let reader = io::BufReader::new(file?).lines();

    let score_map = create_scoring_map();
    let mut score = 0;


    for line in reader{
        match line {
            Result::Ok(value) => {
                let rucksack = parse_line(&value);
                for letter in rucksack.common_in_both_compartments{
                    score += get_score(letter, &score_map);
                }
            }
            Result::Err(_err) => {
                continue;
            }
        }
    }


    println!("Final: {}", score);

    Ok(())
}
