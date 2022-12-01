use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    // let mut file = File::open("src/input").expect("Error opening file");
    // let mut contents = String::new();

    let mut current_elf_calories = 0;
    let mut currently_most_calories = 0;

    let lines = read_lines("src/input").expect("Error opening file");
    for line in lines {
        if let Ok(line_value) = line {

            if line_value.is_empty() {
                println!("Next elf");
                println!("Current elf calories: {}", current_elf_calories);

                if current_elf_calories > currently_most_calories {
                    currently_most_calories = current_elf_calories;
                }

                current_elf_calories = 0;
            }else{
                let current_line_calories_as_int: u32 = line_value.trim().parse().expect("Could not parse line_value into int");
                current_elf_calories += current_line_calories_as_int;

                println!("{}", line_value);
            }
        }
    }

    println!("Most calories: {}", currently_most_calories);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}