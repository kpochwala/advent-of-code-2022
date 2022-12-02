use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    // let mut file = File::open("src/input").expect("Error opening file");
    // let mut contents = String::new();

    let mut current_elf_calories = 0;
    // let mut top_elfes: Vec<u32> = Vec::with_capacity(3);
    let mut top_elfes  = vec![0,0,0];

    let lines = read_lines("src/input").expect("Error opening file");
    for line in lines {
        if let Ok(line_value) = line {

            if line_value.is_empty() {
                println!("Next elf");
                println!("Current elf calories: {}", current_elf_calories);

                if current_elf_calories >= top_elfes[0] {

                    top_elfes.remove(0);
                    top_elfes.insert(0, current_elf_calories);
                    top_elfes.sort();
                    println!("best elves: 0: {} 1: {} 2: {}", top_elfes[0], top_elfes[1], top_elfes[2]);
                }

                current_elf_calories = 0;
            }else{
                let current_line_calories_as_int: u32 = line_value.trim().parse().expect("Could not parse line_value into int");
                current_elf_calories += current_line_calories_as_int;

                // println!("{}", line_value);
            }
        }
    }

    println!("Most calories: {}", top_elfes[0]);

    let mut top_elfes_calories = 0;
    for elf in top_elfes{
        println!("One of the tops: {}", elf);
        top_elfes_calories += elf;
    }

    println!("Top stash calories: {}", top_elfes_calories);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}