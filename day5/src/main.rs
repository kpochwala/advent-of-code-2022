use std::fs::File;
use std::io::{self, Lines, BufReader, BufRead};
use std::str::{self, FromStr};
use std::collections::HashMap;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
#[derive(Clone)]
pub struct Command {
    how_many: u32,
    from: String,
    to: String,
}

#[derive(Debug)]
struct Input {
    initial_stack: Vec<String>,
    commands: Vec<Command>,
    stack_ids: Vec<String>,
    dock_map: HashMap::<String, VecDeque<String>>,
}

impl Input {
    fn new() -> Input {
        Input {
            initial_stack: vec![],
            commands: vec![],
            stack_ids: vec![],
            dock_map: HashMap::new(),
        }
    }
}

pub trait Parseable {
    fn parse(&mut self, reader: Lines<BufReader<File>>) -> io::Result<()>;
}

pub trait Commandable {
    fn command(&mut self, command: Command) -> ();
}

impl Commandable for Input {
    fn command(&mut self, command: Command) -> () {
        for n in 0..command.how_many{
            let container = self.dock_map.get_mut(&command.from).unwrap().pop_front().unwrap();
            self.dock_map.get_mut(&command.to).unwrap().push_front(container);
        }
    }
}

enum FilePart {
    InitialStack,
    Commands
}

impl Parseable for Input {
    fn parse(&mut self, reader: Lines<BufReader<File>>) -> io::Result<()> {
        self.initial_stack = vec![];
        self.commands = vec![];

        let mut file_part = FilePart::InitialStack;

        for maybe_line in reader {
            let line = maybe_line.unwrap();
            
            if line.is_empty() {
                file_part = FilePart::Commands;
                continue;
            }

            match file_part {
                FilePart::InitialStack => self.initial_stack.push(line.clone()),
                FilePart::Commands => {
                    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
                    for capture in re.captures_iter(line.as_str()){
                        self.commands.push(
                            Command{
                                how_many: capture[1].parse::<u32>().unwrap(),
                                from: String::from_str(&capture[2]).unwrap(),
                                to: String::from_str(&capture[3]).unwrap(),
                            }
                        );
                    }
                }
            }
        }

        self.stack_ids = self.initial_stack.pop().unwrap().split_whitespace().map(|x| x.to_string()).collect();
        
        self.dock_map = HashMap::<String, VecDeque<String>>::new();
        for id in &self.stack_ids {
            self.dock_map.insert(id.to_string(), VecDeque::new());
        }
        
        for line in self.initial_stack.iter() {
            let mut split = line.as_bytes().chunks(4)
                .map(str::from_utf8)
                .map(|x| x.unwrap().trim())
                .collect::<Vec<&str>>()
                .clone();

            for id in self.stack_ids.iter().rev() {
                let val = split.pop().unwrap();
                println!("Val: {:?}", val);
                if !val.is_empty(){
                    self.dock_map.get_mut(id).unwrap().push_back(String::from_str(val).unwrap());
                }
            }

        }

        Ok(())
    }
}

fn main()  -> io::Result<()> {
    let file = File::open("src/input");
    let reader = io::BufReader::new(file?).lines();

    let mut input = Input::new();
    input.parse(reader).unwrap();

    let command_clones = input.commands.clone();

    println!("Initial dock map: {:?}", input.dock_map);

    for command in command_clones{
        println!("Executing command {:?}", command);
        input.command(command.clone());
        println!("Dock map after command{:?}", input.dock_map);
        println!();
    }

    println!("Final dock map: {:?}", input.dock_map);

    for id in input.stack_ids{
        // println!("stack id: {}", id);
        let mut txt = input.dock_map.get_mut(&id).unwrap().pop_front().unwrap();
        txt = txt.replace("[", "");
        txt = txt.replace("]", "");
        print!("{}", txt);
    }

    Ok(())
}
