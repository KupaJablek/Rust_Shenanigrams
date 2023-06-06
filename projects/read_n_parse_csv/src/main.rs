// Purpose: figure out how to read and parse a csv.
// maybe put stuff in a struct to hold info?
//
// csv file is in format: firstname,lastname,age

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fmt;

const CSV_FILE: &str = "data.txt";

#[derive(Debug)] // need to derive debug to print struct in println
struct Person {
    firstname: String,
    lastname: String,
    age: u32,
}

impl fmt::Display for Person { // implement display so that custom struct will work in println!("") statements
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}. Age {}", self.firstname, self.lastname, self.age)
    }
}

fn main() {
    println!("Data file will be read, and the contents parsed.");
    println!("");
    println!("Reading data from CSV...\n");

    let people = get_file_data().unwrap();

    let mut people_count = 1;
    for p in people {
        println!("{}. {}", people_count, p);
        people_count += 1;
    };
}

fn get_file_data() -> Result<Vec::<Person>, std::io::Error> {
    let mut _people = Vec::<Person>::new();

    let path = Path::new(CSV_FILE);

    // read data from the file   
    let file = match File::open(path) {
        Err(_why) => Err("couldnt read file, {_why}"),
        Ok(file) => Ok(file),
    };

    // file is open
    // read the contents
    let reader = BufReader::new(file.unwrap());
    println!("Successfully read from file!\n");

    for line in reader.lines() { // reads up until \n
        if let Ok(l) = line { // if OK() let 'l' = line;
            //println!("{l}");
            let split: Vec<_> = l.split(',').collect(); // split() returns an iterator,
                                                        // collect into a vec for access at [index]

            let p = Person {
                firstname: split[0].to_string(),
                lastname: split[1].to_string(),
                age: split[2].parse::<u32>().unwrap(),
            };
            _people.push(p);
        }   
    }

    return Ok(_people);
}
