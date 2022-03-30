#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use std::io::*;

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

        print!("Incorrect Guesses: ");
        for &g in &incorrect_guesses {
            print!("{} ", g.to_string());
        }

        print!("\n");

        for letter in game_string.iter() {
            print!("{} ", letter.to_string());
        }

        print!("\n");

        stdin()
            .read_line(&mut buf)
            .ok()
            .expect("Error reading line")
            .to_string();

        // guess = buf.chars().nth(0).unwrap();
        guess = buf.remove(0);

        println!("{}, {:?}", map.contains_key(&guess), map);

        if map.contains_key(&guess) {
            let num_insertions = *map.get(&guess).unwrap() as i32;

            for _i in 0..num_insertions {
                let pos = *positions.iter().find(|&&x| x.0 == guess).unwrap();
                let index = positions.iter().position(|&x| x == pos).unwrap();
                game_string[pos.1] = String::from(pos.0);
                positions.remove(index);
            }

            println!("Good guess!");
        } else {
            incorrect_guesses.insert(guess);
            counter += 1;
            println!("Wrong letter! You have {} guesses left!", 10 - counter);
        }

        if game_string.join("") == input.to_string() {
            println!("You got it! In {} guesses too!", 10 - counter);
            break;
        } else if counter == 9 {
            println!(
                "😬... Sorry, that's not right, and you're all out of guesses. The word was {}",
                input.to_string()
            );
            break;
        }
    }
}
