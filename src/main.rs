use rand::seq::SliceRandom;
use std::env;

//Some notes on my calculations here. There are 37315 words in the file.
//You can get that by doing: 
//grep -E '^.{1,5}$' words.txt | grep -E '^[a-zA-Z]+$' | sed 's/.*/\L&/' | uniq
//on the original file which is also stored in the assets folder
//
//Assuming a computer can do 100th/s It would take on average 22.9/2 = 11.45 years 
//to crack a 5 word password.
//You can get that by doing: (((37315 ^ 5)) / 100000000000000) / (60 * 60 * 24 * 365)
//
//The word list however is pretty bad, most of the words i've never seen before.
//Would be good to use a list of words that are more common and search for a better
//balance between number of combinations and words that are actually used.
//
//The word list its self is from: https://github.com/dwyl/english-words
//
//Either way it's always nice to write some rust :)

fn main() {
    // Read the text file
    let file_contents = include_bytes!("assets/words.txt");
    let file_str = String::from_utf8_lossy(file_contents);

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // If args is smaller than one print an error and return
    if args.len() < 2 {
        println!("Usage: {} <num_lines> [num_iterations]", args[0]);
        return;
    }

    let num_lines = args[1].parse::<usize>().expect("Invalid input: expected integer");
    let num_iterations = args.get(2).map(|s| s.parse::<usize>().expect("Invalid input: expected integer")).unwrap_or(1);

    // Split the text file into lines
    let lines: Vec<&str> = file_str.lines().collect();

    // Randomly select lines and print the concatenated result
    let mut rng = rand::thread_rng();
    for _ in 0..num_iterations {
        let selected_lines: Vec<&str> = lines
            .choose_multiple(&mut rng, num_lines)
            .cloned()
            .collect();
        
        //convert to lower case
        print!("{}", selected_lines.join(" ").to_lowercase());
        print!("\t");
        println!("{}", selected_lines.join("").to_lowercase());
    }
}
