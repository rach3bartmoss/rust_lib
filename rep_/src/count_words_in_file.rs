use std::env;
///This program reads from a file and return how many times a word is repeated
use std::fs;

fn read_file(file_name: String) -> String {
    let open_file = fs::read_to_string(file_name);
    println!("{}", open_file);
    open_file
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: <program> <file_name>");
        return;
    }
    let filename = &args[1];
    println!("File readed: {filename}");

    //let content = read_file(filename).expect("Should have been able to read the file");
    //println!("The content in file: \n{content}");
}
