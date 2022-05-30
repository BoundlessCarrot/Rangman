/*
NOTE: tutorial video referenced is here - https://www.youtube.com/watch?v=NtUkr_z7l84

Could the print macro (println!) get redirected from the stdout to the ui central panel?
*/
#[allow(unused_imports)]
use eframe::{egui::CentralPanel, egui::Ui, epi::App, run_native, NativeOptions};
use std::collections::{HashMap, HashSet};
use std::io::stdin;

struct GameCore {
    input_string: String,
    game_string: Vec<String>,
    letter_map: HashMap<char, usize>,
    position_map: Vec<(char, usize)>,
    incorrect_guesses: HashSet<char>,
}

// Background stuff
impl GameCore {
    fn new() -> GameCore {
        let mut input = String::new();
        GameCore::get_input(&mut input);
        let length = input.len();
        let maps = GameCore::give_word_info(&input);
        GameCore {
            input_string: input,
            game_string: vec![String::from("_"); length - 1],
            letter_map: maps.0.to_owned(),
            position_map: maps.1.to_owned(),
            incorrect_guesses: HashSet::new(),
        }
    }

    fn single_loop_step(&mut self, mut counter: i32) {
        let mut buf: String = String::new();
        let guess: char;

        // From here
        println!("Guess a letter!");
        print!("Current word: ");
        for letter in &self.game_string {
            print!("{} ", letter.to_string());
        }

        println!();

        print!("Incorrect Guesses: ");
        for g in &self.incorrect_guesses {
            print!("{} ", g.to_string());
        }

        println!();
        // To here is all fluff for the cli, I imagine it won't be  
        // necessary for the UI as it's not part of the game logic

        GameCore::get_input(&mut buf);
        guess = buf.remove(0);
        // Check for membership in the master string thru the map and positions vectors (also gives us access to number of occurances and index)
        if self.letter_map.contains_key(&guess) {
            // This logic could be improved (see the input check in
            //  get_input()). I think that could be applied to this.
            let num_insertions = *self.letter_map.get(&guess).unwrap() as i32;
            for _i in 0..num_insertions {
                let pos = *self.position_map.iter().find(|&&x| x.0 == guess).unwrap();
                let index = self.position_map.iter().position(|&x| x == pos).unwrap();
                self.game_string[pos.1] = String::from(pos.0);
                self.position_map.remove(index);
            }
            println!("Good guess!");
            println!();
        } else {
            self.incorrect_guesses.insert(guess);
            counter -= 1;
            println!("Wrong letter! You have {} guesses left!", counter);
            println!();
        }

        // End game checks
        if self.game_string.join("") == self.input_string.to_string()
            || self.position_map.len() == 0
        {
            println!(
                "You got it! With {} guesses left too! The word was {}",
                counter,
                self.input_string.to_string()
            );
            // break;
        } else if counter == 0 {
            println!(
                "😬... Sorry, that's not right, and you're all out of guesses. The word was {}",
                self.input_string.to_string()
            );
            // break;
        }
    }

    fn get_input(val: &mut String) {
        stdin().read_line(val).ok().expect("Error reading line");


        if !val[0..val.len() - 1].chars().all(char::is_alphabetic) {
            println!("Unrecognized character entered, please try again");
            val.clear();
            GameCore::get_input(val);
        }
    }

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
}

// GUI stuff
#[allow(unused_variables)]
impl App for GameCore {
    

    fn update(&mut self, ctx: &egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Hangman!");
            ui.label(&self.input_string);

            // let state_id = ui.id().with("show_plaintext");
            // let mut show_plaintext = ui.data().get_temp::<bool>(state_id).unwrap_or(false);
            // let result = ui.with_layout(egui::Layout::right_to_left(), |ui| {
            //     let response = ui.add(egui::SelectableLabel::new(show_plaintext, "👁")).on_hover_text("Show/hide word");

            //     if response.clicked() {
            //         show_plaintext = !show_plaintext;
            //     }

            //     ui.add_sized(ui.available_size(), egui::TextEdit::singleline("Enter word").password(!show_plaintext));
            // })

            // ui.label(&self.);

        });
    }

    fn name(&self) -> &str {
        "Rustman"
    }

    fn setup(&mut self, _ctx: &egui::Context, _frame: &eframe::epi::Frame, _storage: Option<&dyn eframe::epi::Storage>) {

    }

    

    // fn save(&mut self, _storage: &mut dyn eframe::epi::Storage) {}

    // fn on_exit_event(&mut self) -> bool {
    //     true
    // }

    // fn on_exit(&mut self) {}

    // fn auto_save_interval(&self) -> std::time::Duration {
    //     std::time::Duration::from_secs(30)
    // }

    // fn max_size_points(&self) -> egui::Vec2 {
    //     egui::Vec2::new(1024.0, 2048.0)
    // }

    // fn clear_color(&self) -> egui::Rgba {
    //     // NOTE: a bright gray makes the shadows of the windows look weird.
    //     // We use a bit of transparency so that if the user switches on the
    //     // `transparent()` option they get immediate results.
    //     egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    // }

    // fn persist_native_window(&self) -> bool {
    //     true
    // }

    // fn persist_egui_memory(&self) -> bool {
    //     true
    // }

    // fn warm_up_enabled(&self) -> bool {
    //     false
    // }
}

fn main() {
    let app = GameCore::new();
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options);

    // let counter: i32 = 10;
    // while counter > 0 {
    //     app.single_loop_step(counter)
    // }
}
