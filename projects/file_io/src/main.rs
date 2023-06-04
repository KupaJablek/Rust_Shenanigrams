use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Writing to 'foo.txt'!");
    create_file()?;
    //read_from_file()?;
    let data: Vec<_> = std::fs::read_to_string("foo.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut i = 1;

    for line in data {
        println!("{i}. {line}");
        let i = i += 1;
    }

    Ok(())
}

fn create_file() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello from foo.txt\nhaha secret second line")?;
    Ok(())
}

fn read_from_file() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?; 

    // data is attained in a vec format
    //let mut data = vec![];

    //file.read_to_end(&mut data)?;
    // reads all data to one string
    let data = std::fs::read_to_string("foo.txt")?; 
    //iterate and print vec? 
    //for i in &data { 
    //    println!("{i}");
    //}
    println!("{data}");

    Ok(())
}

