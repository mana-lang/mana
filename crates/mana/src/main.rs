use mana::test;
use std::fs::File;
use std::io::prelude::*;

fn read() -> Result<String, std::io::Error> {
    let mut file = File::open("crates/mana/examples/hello.mana")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let r = read();
    match r {                                                
        Ok(st) => println!("{}", st),                                                  
        Err(error) => {                                                    
            panic!("Problem opening the file: {:?}", error)                
        },                                                                 
    }
    test()
}