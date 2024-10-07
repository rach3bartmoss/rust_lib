use std::env;
use std::fs;

///This program reads from a file and return how many times a word is repeated
fn read_file(file_name: &String) -> String {
    let open_file = fs::read_to_string(file_name);
    open_file.expect("Error reading the file")
}

fn count_word_in_file(readed_file: &String, target_word: &String) -> u32 {
    let count: u32 = 0;
    while readed_file.find(target_word) > 0 {
        count += 1;
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: <program> <file_name> <target_word>");
        return;
    }
    let filename = &args[1];
    let target = &args[2];
    println!("File readed: {filename}");

    let content = read_file(filename);
    let reps = count_word_in_file(&content, target);
    println!(
        "The content in file: \n{content}\n\nNumber of reps: {}",
        reps
    );
}
