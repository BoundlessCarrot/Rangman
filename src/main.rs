#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::io::stdin;

fn give_word_info(word: &String) -> (HashMap<char, usize>, Vec<(char, usize)>) {
    let mut letter_counts = HashMap::new();
    let mut letter_positions = vec![];

    for (i, letter) in word.chars().enumerate() {
        if letter != '\n' {
            letter_counts.insert(letter.to_owned() as char, word.matches(letter).count());
            letter_positions.push((letter, i))
        }
    }

    return (letter_counts, letter_positions);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error reading line");
    let mut buf: String = String::new();
    let mut guess: char;
    let mut game_string = vec![String::from("_"); input.len() - 1];
    let mut incorrect_guesses: HashSet<char> = HashSet::new();

    let maps = give_word_info(&input);
    let map = maps.0;
    let mut positions = maps.1;

    let mut counter: i32 = 0;

    while counter < 10 {
        println!("Guess a letter!");

        print!("Current word: ");
        for letter in game_string.iter() {
            print!("{} ", letter.to_string());
        }

        println!();

        print!("Incorrect Guesses: ");
        for g in &incorrect_guesses {
            print!("{} ", g.to_string());
        }
        
        println!();

        stdin()
            .read_line(&mut buf)
            .ok()
            .expect("Error reading line")
            .to_string();

        guess = buf.remove(0);
        
        // Check for membership in the master string thru the map and positions vectors (also gives us access to number of occurances and index)
        if map.contains_key(&guess) {
            let num_insertions = *map.get(&guess).unwrap() as i32;

            for _i in 0..num_insertions {
                let pos = *positions.iter().find(|&&x| x.0 == guess).unwrap();
                let index = positions.iter().position(|&x| x == pos).unwrap();
                game_string[pos.1] = String::from(pos.0);
                positions.remove(index);
            }

            println!("Good guess!");
            println!();
        } else {
            incorrect_guesses.insert(guess);
            counter += 1;
            println!("Wrong letter! You have {} guesses left!", 10 - counter);
            println!();
        }

        // println!("{}, {:?}, {:?}, {}", map.contains_key(&guess), incorrect_guesses, positions, game_string.join("")); --> debug string

        // End game checks
        if game_string.join("") == input.to_string() || positions.len() == 0 {
            println!("You got it! In {} guesses too!", 10 - counter);
            break;
        } else if counter == 10 {
            println!(
                "😬... Sorry, that's not right, and you're all out of guesses. The word was {}",
                input.to_string()
            );
            break;
        }

        // Empty buffer
        buf = String::new();
    }
}
