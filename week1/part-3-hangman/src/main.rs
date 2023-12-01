// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    let mut counter = 0;
    let mut word_so_far: Vec<char> = vec!['-';secret_word_chars.len()];
    println!("Welcome to CS110L Hangman!");
    let mut guessed = String::new();

    loop {
        println!("The word so far is {}", word_so_far.iter().collect::<String>());
        for w in word_so_far.iter() {
            if *w != '-' && !guessed.contains(*w) {
                guessed.push(*w);
            }
        }
        println!("You have guessed the following letters: {}", guessed);
        println!("You have {} guesses left", NUM_INCORRECT_GUESSES - counter);
        print!("Please guess a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        let guess_ch: char = match io::stdin().read_line(&mut guess) {
            Ok(n) if n == 2 => guess.chars().next().unwrap(),
            _ => continue,
        };

        if secret_word_chars.contains(&guess_ch) {
            for (i, e) in secret_word_chars.iter().enumerate() {
                if *e == guess_ch {
                    word_so_far[i] = *e;
                }
            }
            if !word_so_far.contains(&'-') {
                println!("\nCongratulations you guessed the secret word: {}!", secret_word);
                break;
            }
        } else {
            counter += 1;
            if counter >= NUM_INCORRECT_GUESSES {
                println!("\nSorry, you ran out of guesses!");
                break;
            }
            println!("Sorry, that letter is not in the word");
        }
        println!("");
    }
}
