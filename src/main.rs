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
    let file_contents = include_bytes!("assets/words-10000.txt");
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

    let word_list_size = lines.len() as f64;
    let selected_words = num_lines as f64;
    //100 trillion checks per second
    let power = 100_000_000_000_000.0 as f64;
    let permutations = word_list_size.powf(selected_words);
    let time_to_crack_seconds = (permutations as f64) / power;
    let seconds_per_year = (60 * 60 * 24 * 365) as f64;
    //On average it will take half the time it takes to check all the permutations
    let time_to_crack_years = time_to_crack_seconds / (2.0 * seconds_per_year);
    println!();
    println!("Selected {} words out of a list of size {}", selected_words, word_list_size);
    println!("At 100 trillion checks per second this password could be cracked in {} years", time_to_crack_years);
    println!("At 1 billion checks per second this password could be cracked in {} years", time_to_crack_years * 100_000.0);
    println!("At 1 million checks per second this password could be cracked in {} years", time_to_crack_years * 100_000_000.0);

}
