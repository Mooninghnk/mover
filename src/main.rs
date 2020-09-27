use std::fs::File;
use std::io::prelude::*;


enum Handeler{
    Success,
    Fail,
}



fn main() {
    let args: Vec<String> = std::env::args().collect();

    for s in args.iter() {
        if s == "read" {
            let x = read_file(String::from(&args[2]));
            println!("{}", x);
            break;
        } else if s == "create" {
            let suc = create_file(String::from(&args[2]), String::from(&args[3]));
            match suc {
                Handeler::Success => println!("seccesfully wrote a new file"),
                Handeler::Fail =>  break,
            }
        };  
    };

}

fn read_file(x: String) -> String {
    let mut file = match File::open(x) {
        Ok(f) => f,
        Err(e) => panic!("Problem reading the file {}", e),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    return contents;
}

fn create_file(name: String, to_write: String) -> Handeler {
    let mut file = match File::create(name) {
        Ok(x) => x,
        Err(e) => panic!("Problem creating the file {}", e),
    };
    match file.write_all(to_write.as_bytes()) {
        Ok(s) => return Handeler::Success,
        Err(e) => return Handeler::Fail,
    };
}