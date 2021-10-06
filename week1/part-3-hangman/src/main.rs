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

fn get_input() -> char {
    print!("Please guess a letter: ");
    io::stdout()
        .flush()
        .expect("Error flushing stdout.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading line.");
    guess.trim().parse().unwrap()
}

fn process(guessed: &mut u8, remaining: &mut u32 ,current: & mut Vec<char>, letters: & mut Vec<char>, secret_word_chars:  &Vec<char>) {
    println!("The word so far is {}", current.iter().collect::<String>());
    println!("You have guessed the following letters: {}", letters.iter().collect::<String>());
    println!("You have {} guesses left", remaining);
    let input = get_input();
    let mut has_guessed = false;
    letters.push(input);

    for (i,letter) in secret_word_chars.iter().enumerate() {
        if *letter == input {
            current[i] = *letter;
            *guessed += 1;
            has_guessed = true;
        }
    }

    if !has_guessed {
        *remaining -= 1;
        println!("Sorry, that letter is not in the word");
    }
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut result = vec!['-'; secret_word_chars.len()];
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut guessed: u8 = 0;
    let mut guess_remaining = NUM_INCORRECT_GUESSES;
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    println!("Welcome to CS110L Hangman!");
    // Your code here! :)

    loop {
        process(& mut guessed, &mut guess_remaining, & mut result, & mut guessed_letters, &secret_word_chars);
        if guessed == secret_word_chars.len() as u8 || guess_remaining == 0 {
            break
        }
    }

}
