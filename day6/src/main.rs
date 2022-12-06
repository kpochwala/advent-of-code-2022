
#![feature(utf8_chunks)]
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn has_unique_elements(elements: &[u8]) -> bool {
    let mut uniq = HashSet::new();
    elements.into_iter().all(move |x| uniq.insert(x))
}
fn main() -> std::io::Result<()> {

    let mut input = String::new();

    File::open("src/input")?.read_to_string(&mut input);
    let iter = input.as_bytes().to_vec();

    let window_size = 14;

    println!("Input: {:?}", input);
    let mut counter = window_size;
    for chunk in iter.windows(window_size) {
        
        // println!("chunk: {:?}, index: {:?}, unique: {:?}", String::from_utf8(chunk.to_vec()), counter, has_unique_elements(chunk));
        if has_unique_elements(chunk){
            println!("Unique: {:?}", counter);
            break;
        }

        counter += 1;

    }

    println!("Hello, world!");
    Ok(())
}
