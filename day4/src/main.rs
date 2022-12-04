use std::fs::File;
use std::io::{self, BufRead};
use array_tool::vec::Intersect;

fn main() -> io::Result<()> {

    let file = File::open("src/input");
    let mut reader = io::BufReader::new(file?).lines();

    let mut overlap_counter = 0;
    let mut partial_overlap_counter = 0;

    for maybe_line in reader {
        let line =  maybe_line.unwrap();
        let sections: Vec<&str> = line.split(",").collect();
        let expanded_sections: Vec<u32> = vec![];

        let mut section_numbers: Vec<Vec<u32>> = vec![];
        
        for section in &sections {
            let section_range: Vec<&str> = section.split("-").collect();
            let range_start = section_range[0].parse::<u32>().unwrap();
            let range_stop = section_range[1].parse::<u32>().unwrap();

            let section_values: Vec<u32> = (range_start..=range_stop).collect();
            section_numbers.push(section_values);
        }

        let first = section_numbers[0].clone();
        let second = section_numbers[1].clone();
        let common = section_numbers.pop().unwrap().intersect(section_numbers.pop().unwrap());

        if common.len() != 0 {
            partial_overlap_counter+=1;
        }

        if first == common {
            overlap_counter+=1;
        }else if second == common {
            overlap_counter+=1;
        }

        println!("Line: {:?}", sections);
        println!("Common: {:?}", common);
    }

    let mut score = 0;


    println!("Overlap counter: {}", overlap_counter);
    println!("Partial overlap counter: {}", partial_overlap_counter);

    Ok(())
}
